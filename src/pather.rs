use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use image::GrayImage;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use crate::blueprint::Blueprint;
use crate::line::Line;
use crate::peg::{Peg, Yarn};
use crate::utils;

#[derive(Debug, Clone)]
pub struct PatherConfig {
    /// Number of [`Peg`] connections.
    pub iterations: u32,
    /// How much to lighten the the pixels at each pass, between 0 and 1.
    /// Low values encourage line overlap.
    pub line_opacity: f64,
    /// Radius around [`Pegs`](Peg), in pixels, to use to determine the starting [`Peg`].
    pub start_peg_radius: u32,
    /// Don't connect [`Pegs`](Peg) within distance, in pixels.
    pub skip_peg_within: u32,
    /// Display progress bar.
    pub progress_bar: bool,
}

impl PatherConfig {
    /// Creates a new [`PatherConfig`].
    pub fn new(
        iterations: u32,
        line_opacity: f64,
        start_peg_radius: u32,
        skip_peg_within: u32,
        progress_bar: bool,
    ) -> Self {
        Self {
            iterations,
            line_opacity,
            start_peg_radius,
            skip_peg_within,
            progress_bar,
        }
    }
}

impl Default for PatherConfig {
    fn default() -> Self {
        Self {
            iterations: 4000,
            line_opacity: 0.4,
            start_peg_radius: 5,
            skip_peg_within: 0,
            progress_bar: false,
        }
    }
}

#[derive(Debug)]
pub struct Pather {
    /// Input grayscale image.
    pub image: GrayImage,
    /// [`Peg`] vector.
    pub pegs: Vec<Peg>,
    /// [`Yarn`], only the width field is important to compute the [`Blueprint`].
    pub yarn: Yarn,
    /// [`PatherConfig`], algorithm config.
    pub config: PatherConfig,
    /// Holds the pixel coords of all the lines
    line_cache: HashMap<(u16, u16), Line>,
}

impl Pather {
    /// Creates a new [`Pather`].
    pub fn new(img: GrayImage, pegs: Vec<Peg>, yarn: Yarn, config: PatherConfig) -> Self {
        let line_cache = HashMap::new();
        Self {
            image: img,
            pegs,
            yarn,
            config,
            line_cache,
        }
    }

    /// Creates a nes [`Pather`] from an image file.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`image::open`] fails to open the image file.
    pub fn from_image_file(
        image_path: PathBuf,
        pegs: Vec<Peg>,
        yarn: Yarn,
        config: PatherConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let img = image::open(image_path)?.into_luma8();
        Ok(Self::new(img, pegs, yarn, config))
    }

    /// Populate the `line_cache` with the pixel coords of all the lines between the [`Peg`] pairs.
    pub fn populate_line_cache(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Populating line cache");

        let peg_combinations = self
            .pegs
            .iter()
            .tuple_combinations()
            .filter(|(peg_a, peg_b)| peg_a.dist_to(peg_b) >= self.config.skip_peg_within)
            .collect_vec();

        let pbar = utils::pbar(peg_combinations.len() as u64, !self.config.progress_bar)?
            .with_message("Populating line cache");

        let key_line_pixels = peg_combinations
            .par_iter()
            .progress_with(pbar)
            .map(|(peg_a, peg_b)| (utils::hash_key(peg_a, peg_b), peg_a.line_to(peg_b, 1.)))
            .collect::<Vec<((u16, u16), Line)>>();

        for (key, line) in key_line_pixels {
            self.line_cache.insert(key, line);
        }
        debug!("# line cache entries: {}", self.line_cache.len());
        Ok(())
    }

    /// Get starting peg by taking the [`Peg`] located on the darkest pixel.
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

        let min_index = peg_avgs.iter().position_min().unwrap_or(0);

        &self.pegs[min_index]
    }

    /// Compute the [`Blueprint`].
    pub fn compute(&self) -> Result<Blueprint, Box<dyn Error>> {
        let start_peg = self.get_start_peg(self.config.start_peg_radius);
        info!("starting peg: {start_peg:?}");
        let mut peg_order = vec![start_peg];
        let mut work_img = self.image.clone();

        let pbar = utils::pbar(self.config.iterations as u64, !self.config.progress_bar)?
            .with_message("Computing blueprint");

        let line_color = 255. * self.config.line_opacity;
        let opacity_factor = 1. - self.config.line_opacity;

        let mut last_peg = start_peg;
        let mut last_last_peg = last_peg;

        // Use a ThreadPool to reduce overhead
        let pool = ThreadPoolBuilder::new().build().unwrap();

        pool.install(|| {
            for _ in pbar.wrap_iter(0..self.config.iterations) {
                let (min_loss, min_peg, min_line) = self
                    .pegs
                    .par_iter()
                    .filter(|peg| peg.id != last_peg.id && peg.id != last_last_peg.id)
                    .filter_map(|peg| {
                        let line = self.line_cache.get(&utils::hash_key(last_peg, peg))?;
                        let loss = line.loss(&work_img);
                        Some((loss, peg, line))
                    })
                    .min_by(|(loss1, _, _), (loss2, _, _)| {
                        loss1
                            .partial_cmp(loss2)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap();

                debug!("line {:?} -> {:?}: {min_loss:?}", last_peg.id, min_peg.id);
                peg_order.push(min_peg);
                last_last_peg = last_peg;
                last_peg = min_peg;

                min_line.zip().for_each(|(x, y)| {
                    let pixel = work_img.get_pixel_mut(*x, *y);
                    pixel.0[0] = ((opacity_factor) * pixel.0[0] as f64 + line_color)
                        .round()
                        .min(255.0) as u8;
                });
            }
        });

        Ok(Blueprint::from_refs(
            peg_order,
            self.image.width(),
            self.image.height(),
            Some((255, 255, 255)),
            1.,
        ))
    }
}
