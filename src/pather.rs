use image::GrayImage;
use log::{debug, info};
use std::collections::HashMap;
use std::path::PathBuf;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle};
use svg::{Document, Node};

use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;

use crate::peg::{Blueprint, Line, Peg, Yarn};
use crate::utils;

#[derive(Debug)]
pub struct PatherConfig {
    pub iterations: u32,
    pub lighten_factor: f64,
    pub start_peg_radius: u32,
    pub skip_peg_within: u32,
    pub progress_bar: bool,
}

impl PatherConfig {
    pub fn new(
        iterations: u32,
        lighten_factor: f64,
        start_peg_radius: u32,
        skip_peg_within: u32,
        progress_bar: bool,
    ) -> Self {
        Self {
            iterations,
            lighten_factor,
            start_peg_radius,
            skip_peg_within,
            progress_bar,
        }
    }

    pub fn new_with_defaults() -> Self {
        Self {
            iterations: 10000,
            lighten_factor: 1.05,
            start_peg_radius: 5,
            skip_peg_within: 100,
            progress_bar: false,
        }
    }
}

#[derive(Debug)]
pub struct Pather {
    pub image: GrayImage,
    pub pegs: Vec<Peg>,
    pub yarn: Yarn,
    pub config: PatherConfig,
    /// Holds the pixel coords of all the lines
    line_cache: HashMap<(u16, u16), Line>,
}

impl Pather {
    pub fn new(img: GrayImage, pegs: Vec<Peg>, yarn: Yarn, config: PatherConfig) -> Self {
        let line_cache = HashMap::new();
        let mut out = Self {
            image: img,
            pegs,
            yarn,
            config,
            line_cache,
        };
        out.populate_line_cache();
        out
    }

    pub fn from_file(
        image_path: PathBuf,
        pegs: Vec<Peg>,
        yarn: Yarn,
        config: PatherConfig,
    ) -> Self {
        let img = image::open(image_path).unwrap().into_luma8();
        Self::new(img, pegs, yarn, config)
    }

    /// Populate the [line_cache] with the pixel coords of all the line between the peg pairs
    fn populate_line_cache(&mut self) {
        info!("Populating line cache");

        let pbar = self.spinner().with_message("Populating line cache");
        for (peg_a, peg_b) in pbar.wrap_iter(self.pegs.iter().tuple_combinations()) {
            self.line_cache.insert(
                self.hash_key(peg_a, peg_b),
                peg_a.line_to(peg_b).with_width(self.yarn.width),
            );
        }
        debug!("# line cache entries: {:?}", self.line_cache.len());
    }

    /// Get starting peg by taking the peg located on the darkest pixel
    fn get_start_peg(&self, radius: u32) -> &Peg {
        let peg_avgs: Vec<u32> = self
            .pegs
            .iter()
            .map(|peg| {
                // get the average pixel value around peg
                let (x_coords, y_coords) = peg.around(radius);
                let pixels: Vec<u8> = x_coords
                    .into_iter()
                    .zip(y_coords)
                    .map(|(x, y)| match self.image.get_pixel_checked(x, y) {
                        Some(pixel) => pixel[0],
                        None => 0,
                    })
                    .collect();
                pixels.iter().fold(0, |acc, pixel| acc + *pixel as u32) / pixels.len() as u32
            })
            .collect();
        debug!("peg_avgs: {peg_avgs:?}");
        let min_index = peg_avgs.iter().position_min().unwrap();
        &self.pegs[min_index]
    }

    /// Compute the peg order
    pub fn peg_order(&self) -> Blueprint {
        // Algorithm:
        //     peg_1 = pegs[0]
        //     output = [peg_1]
        //     for 0..iterations
        //         line_values = []
        //         search_pegs = {pegs} - {peg_1}
        //         for peg in search_pegs
        //             line = peg_1.line_to(peg)
        //             pixels = image[line]
        //             line_values.push(pixels.avg())
        //         next_peg = search_pegs[line_values.argmax()]
        //         output.append(next_peg)
        //         peg_1 = next_peg

        // let yarn_delta = self.yarn.delta() as u16;
        let opacity = 1. - self.config.lighten_factor;
        let layer_delta = 255. * self.config.lighten_factor;

        let max_dist = self
            .line_cache
            .values()
            .map(|line| line.dist)
            .max()
            .unwrap();
        debug!("max_dist: {max_dist:?}");

        let start_peg = self.get_start_peg(self.config.start_peg_radius);
        info!("starting peg: {start_peg:?}");
        let mut peg_order = vec![start_peg];
        let mut work_img = self.image.clone();

        let mut min_loss: f64;
        let mut min_line: Option<&Line>;
        let mut min_peg: Option<&Peg>;

        let pbar = self.pbar().with_message("Computing peg order");

        for _ in pbar.wrap_iter(0..self.config.iterations) {
            min_loss = f64::MAX;
            min_peg = None;
            min_line = None;

            let last_peg = peg_order.get(peg_order.len() - 1).unwrap();
            let last_last_peg = peg_order.get(peg_order.len() - 2).unwrap_or(last_peg);

            for peg in &self.pegs {
                // don't connect to same peg or the previous one
                if peg.id == last_peg.id || peg.id == last_last_peg.id {
                    continue;
                }
                let line = self.line_cache.get(&self.hash_key(last_peg, peg)).unwrap();

                if line.dist <= self.config.skip_peg_within {
                    continue;
                }

                let loss = line
                    .zip()
                    .map(|(x, y)| work_img.get_pixel(*x, *y))
                    .fold(0.0, |acc, &pixel| acc + (pixel.0[0] as f64))
                    / (255. * line.len() as f64);
                // - ALPHA * f64::from(line.dist / max_dist);
                // debug!("loss {:?}", loss);
                if loss < min_loss {
                    min_loss = loss;
                    min_line = Some(line);
                    min_peg = Some(peg);
                }
            }
            // debug!("loss {min_loss:?}");
            peg_order.push(min_peg.unwrap());
            // Update the work img to reflect the added line
            // https://docs.rs/image/latest/image/struct.ImageBuffer.html
            min_line.unwrap().zip().for_each(|(x, y)| {
                let mut pixel = work_img.get_pixel_mut(*x, *y);
                pixel.0[0] = (opacity * pixel.0[0] as f64 + layer_delta).min(255.) as u8;
                // pixel.0[0] = cmp::min(pixel.0[0] as u16 + yarn_delta, 255) as u8;
            });
        }
        Blueprint::from_refs(peg_order, self.image.width(), self.image.height())
    }

    /// Get HashMap key for peg pair irrespective of order
    fn hash_key(&self, peg_a: &Peg, peg_b: &Peg) -> (u16, u16) {
        if peg_a.id < peg_b.id {
            (peg_a.id, peg_b.id)
        } else {
            (peg_b.id, peg_a.id)
        }
    }

    /// Render the blueprint as a raster image.
    ///
    /// # Arguments
    ///
    /// * `blueprint`- The order with which to connect the pegs.
    pub fn render(&self, blueprint: &Blueprint) -> GrayImage {
        // Create white img
        let mut img = image::GrayImage::new(self.image.width(), self.image.height());
        for (_, _, pixel) in img.enumerate_pixels_mut() {
            pixel.0[0] = 255;
        }

        let opacity = 1. - self.yarn.opacity;

        let pbar = self.pbar().with_message("Rendering img");

        // Iterate with pairs of consecutive pegs
        for (peg_a, peg_b) in pbar.wrap_iter(blueprint.zip()) {
            let line = self.line_cache.get(&self.hash_key(peg_a, peg_b)).unwrap();
            // let line = peg_a.line_to(peg_b);
            line.zip().for_each(|(x, y)| {
                let mut pixel = img.get_pixel_mut(*x, *y);
                // pixel.0[0] = (pixel.0[0] as f64 * 0.99).floor() as u8;
                pixel.0[0] = (pixel.0[0] as f64 * opacity).round() as u8;
            })
        }
        img
    }

    /// Render the blueprint as a svg.
    ///
    /// # Arguments
    ///
    /// * `blueprint`- The order with which to connect the pegs.
    pub fn render_svg(&self, blueprint: &Blueprint) -> Document {
        let mut document = Document::new()
            .set("viewbox", (0, 0, self.image.width(), self.image.height()))
            .set("width", self.image.width())
            .set("height", self.image.height());

        let background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "white");
        document.append(background);

        for (peg_a, peg_b) in blueprint
            .zip()
            .progress()
            .with_message("Rendering svg")
            .with_style(utils::progress_style())
        {
            let data = Data::new()
                .move_to((peg_a.x, peg_a.y))
                .line_to((peg_b.x, peg_b.y));
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", self.yarn.width)
                .set("opacity", self.yarn.opacity)
                .set("d", data);
            document.append(path);
        }
        document
    }

    fn pbar(&self) -> ProgressBar {
        if self.config.progress_bar {
            ProgressBar::new(self.config.iterations as u64)
        } else {
            ProgressBar::hidden()
        }
        .with_style(utils::progress_style())
    }

    fn spinner(&self) -> ProgressBar {
        if self.config.progress_bar {
            ProgressBar::new_spinner()
        } else {
            ProgressBar::hidden()
        }
    }
}
