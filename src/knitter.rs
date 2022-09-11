use image::GrayImage;
use log::{debug, info};
use std::path::PathBuf;
use std::{cmp, collections::HashMap};

use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::peg::{Line, Peg, Yarn};
use crate::utils;

#[derive(Debug)]
pub struct KnitterConfig {
    pub iterations: u32,
    pub lighten_factor: f64,
    pub exclude_neighbours: u16,
}

impl KnitterConfig {
    pub fn new(iterations: u32, lighten_factor: f64, exclude_neighbours: u16) -> Self {
        Self {
            iterations,
            lighten_factor,
            exclude_neighbours,
        }
    }

    pub fn new_with_defaults(n_pegs: usize) -> Self {
        Self {
            iterations: 10000,
            lighten_factor: 1.05,
            exclude_neighbours: (n_pegs / 20) as u16,
        }
    }
}

#[derive(Debug)]
pub struct Knitter {
    pub image: GrayImage,
    pub pegs: Vec<Peg>,
    pub yarn: Yarn,
    pub config: KnitterConfig,
    /// Holds the pixel coords of all the lines
    line_cache: HashMap<(u16, u16), Line>,
}

impl Knitter {
    pub fn new(img: GrayImage, pegs: Vec<Peg>, yarn: Yarn, config: KnitterConfig) -> Self {
        let line_cache = HashMap::new();
        let mut out = Self {
            image: img,
            pegs,
            yarn,
            config,
            line_cache,
        };
        out.populate_line_cache();
        return out;
    }

    pub fn from_file(
        image_path: PathBuf,
        pegs: Vec<Peg>,
        yarn: Yarn,
        config: KnitterConfig,
    ) -> Self {
        let img = image::open(image_path).unwrap().into_luma8();
        Self::new(img, pegs, yarn, config)
    }

    /// Populate the [line_cache] with the pixel coords of all the line between the peg pairs
    fn populate_line_cache(&mut self) {
        for (peg_a, peg_b) in self.pegs.iter().tuple_combinations() {
            self.line_cache
                .insert(self.hash_key(peg_a, peg_b), peg_a.line_to(peg_b));
        }
        debug!("# line cache entries: {:?}", self.line_cache.len());
    }

    /// Get starting peg by taking the peg located on the darkest pixel
    fn get_start_peg(&self) -> &Peg {
        //TODO: probably best to check the region around the pixel...
        let peg_pixels: Vec<u8> = self
            .pegs
            .iter()
            .map(|peg| self.image.get_pixel(peg.x, peg.y)[0])
            .collect();
        let min_index = peg_pixels.iter().position_min().unwrap();
        &self.pegs[min_index]
    }

    /// Compute the peg order
    pub fn peg_order(&self) -> Vec<&Peg> {
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
        let wrap_neighbour = self.pegs.len() as u16 - self.config.exclude_neighbours;
        let max_dist = self
            .line_cache
            .values()
            .map(|line| line.dist)
            .max()
            .unwrap();
        debug!("max_dist: {max_dist:?}");

        let start_peg = self.get_start_peg();
        info!("starting peg {start_peg:?}");
        let mut peg_order = vec![start_peg];
        let mut work_img = self.image.clone();

        let mut min_loss: f64;
        let mut min_line: Option<&Line>;
        let mut min_peg: Option<&Peg>;

        for _ in (0..self.config.iterations)
            .progress()
            .with_message("Computing peg order")
            .with_style(utils::progress_style())
        {
            min_loss = f64::MAX;
            min_peg = None;
            min_line = None;
            let last_peg = peg_order.last().unwrap();

            for peg in &self.pegs {
                let abs_diff = utils::abs_diff(peg.id, last_peg.id);
                if abs_diff <= self.config.exclude_neighbours || abs_diff >= wrap_neighbour {
                    continue;
                }

                let line = self.line_cache.get(&self.hash_key(last_peg, peg)).unwrap();
                let loss = line
                    .zip()
                    .map(|(x, y)| work_img.get_pixel(*x, *y))
                    .fold(0.0, |acc, &pixel| acc + (pixel.0[0] as f64))
                    / (255. * line.len() as f64);
                // - ALPHA * f64::from(line.dist / max_dist);
                // debug!("loss {:?}", loss);
                if loss < min_loss {
                    min_loss = loss;
                    min_line = Some(line.clone());
                    min_peg = Some(&peg);
                }
            }
            peg_order.push(min_peg.unwrap());
            // Update the work img to reflect the added line
            // https://docs.rs/image/latest/image/struct.ImageBuffer.html
            min_line.unwrap().zip().for_each(|(x, y)| {
                let mut pixel = work_img.get_pixel_mut(*x, *y);
                pixel.0[0] =
                    (cmp::max(pixel.0[0], 1) as f64 * self.config.lighten_factor).ceil() as u8;
                // pixel.0[0] = cmp::min(pixel.0[0] as u16 + yarn_delta, 255) as u8;
            });
        }
        // work_img.save("work_img.png").unwrap();
        peg_order
    }

    /// Get HashMap key for peg pair irrespective of order
    fn hash_key(&self, peg_a: &Peg, peg_b: &Peg) -> (u16, u16) {
        if peg_a.id < peg_b.id {
            return (peg_a.id, peg_b.id);
        } else {
            return (peg_b.id, peg_a.id);
        }
    }

    /// Generate the knitart based on the provided peg order
    ///
    /// # Arguments
    ///
    /// * `peg_order`- The order with which to connect the pegs.
    pub fn knit(&self, peg_order: &Vec<&Peg>) -> image::GrayImage {
        // Create white img
        let mut img = image::GrayImage::new(self.image.width(), self.image.height());
        for (_, _, pixel) in img.enumerate_pixels_mut() {
            pixel.0[0] = 255;
        }

        let yarn_delta = self.yarn.delta() as i16;

        // Iterate with pairs of consecutive pegs
        for (peg_a, peg_b) in peg_order
            .iter()
            .progress()
            .with_message("Knitting")
            .with_style(utils::progress_style())
            .zip(peg_order.iter().skip(1))
        {
            peg_a.line_to(peg_b).zip().for_each(|(x, y)| {
                let mut pixel = img.get_pixel_mut(*x, *y);
                // pixel.0[0] = (pixel.0[0] as f64 * DARKEN_FACTOR).floor() as u8;
                pixel.0[0] = cmp::max(pixel.0[0] as i16 - yarn_delta, 0) as u8;
            })
        }
        img
    }
}
