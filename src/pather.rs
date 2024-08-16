use log::warn;
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
pub struct EarlyStopConfig {
    pub loss_threshold: Option<f64>,
    pub max_count: u32,
}

impl Default for EarlyStopConfig {
    fn default() -> Self {
        Self {
            loss_threshold: None,
            max_count: 100,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PatherConfig {
    /// Number of [`Peg`] connections.
    pub iterations: u32,
    /// The [`Yarn`] to use when computing the path.
    pub yarn: Yarn,
    /// [`EarlyStopConfig`].
    pub early_stop: EarlyStopConfig,
    /// Radius around [`Pegs`](Peg), in pixels, to use to determine the starting [`Peg`].
    pub start_peg_radius: u32,
    /// Don't connect [`Pegs`](Peg) within distance, in pixels.
    pub skip_peg_within: u32,
    /// Beam search width.
    pub beam_width: usize,
    /// Display progress bar.
    pub progress_bar: bool,
}

impl PatherConfig {
    /// Creates a new [`PatherConfig`].
    pub fn new(
        iterations: u32,
        yarn: Yarn,
        early_stop: EarlyStopConfig,
        start_peg_radius: u32,
        skip_peg_within: u32,
        beam_width: usize,
        progress_bar: bool,
    ) -> Self {
        Self {
            iterations,
            yarn,
            early_stop,
            start_peg_radius,
            skip_peg_within,
            progress_bar,
            beam_width,
        }
    }
}

impl Default for PatherConfig {
    fn default() -> Self {
        Self {
            iterations: 4000,
            yarn: Yarn::default(),
            early_stop: EarlyStopConfig::default(),
            start_peg_radius: 5,
            skip_peg_within: 0,
            progress_bar: false,
            beam_width: 1,
        }
    }
}

#[derive(Debug, Clone)]
struct BeamState {
    peg_order: Vec<usize>,
    loss: f64,
    image: image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
}

impl Eq for BeamState {}

impl PartialEq for BeamState {
    fn eq(&self, other: &Self) -> bool {
        self.loss == other.loss
    }
}

impl PartialOrd for BeamState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BeamState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We want the lowest loss to be considered "greater" for purposes of sorting
        // so that the smallest loss comes first.
        self.loss
            .partial_cmp(&other.loss)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug)]
/// The line pathing algorithm.
pub struct Pather {
    /// Input grayscale image.
    pub image: GrayImage,
    /// [`Peg`] vector.
    pub pegs: Vec<Peg>,
    /// [`PatherConfig`], algorithm config.
    pub config: PatherConfig,
    /// Holds the pixel coords of all the lines
    line_cache: HashMap<(u16, u16), Line>,
}

impl Pather {
    /// Creates a new [`Pather`].
    pub fn new(img: GrayImage, pegs: Vec<Peg>, config: PatherConfig) -> Self {
        let line_cache = HashMap::new();
        Self {
            image: img,
            pegs,
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
        config: PatherConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let img = image::open(image_path)?.into_luma8();
        Ok(Self::new(img, pegs, config))
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
            .map(|(peg_a, peg_b)| {
                (
                    utils::hash_key(peg_a, peg_b),
                    peg_a.line_to(peg_b, self.config.yarn.width),
                )
            })
            .collect::<Vec<((u16, u16), Line)>>();

        for (key, line) in key_line_pixels {
            self.line_cache.insert(key, line);
        }
        debug!("# line cache entries: {}", self.line_cache.len());
        Ok(())
    }

    /// Get starting peg by taking the [`Peg`] located on the darkest pixel.
    fn get_start_peg(&self, radius: u32) -> usize {
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

        peg_avgs.iter().position_min().unwrap_or(0)
    }

    fn early_stop(&self, count: &mut u32, loss: f64) -> bool {
        match self.config.early_stop.loss_threshold {
            Some(early_stop_count) => {
                if loss > early_stop_count {
                    *count += 1;
                    debug!(
                        "Early stop count to {}/{}",
                        count, self.config.early_stop.max_count
                    );
                    *count >= self.config.early_stop.max_count
                } else {
                    *count = 0;
                    false
                }
            }
            None => false,
        }
    }

    /// Run the line pathing algorithm and contruct a [`Blueprint`].
    pub fn compute_greedy(&self) -> Result<Blueprint, Box<dyn Error>> {
        if self.line_cache.is_empty() {
            return Err("Line cache is empty, run 'populate_line_cache'.".into());
        };

        let start_peg = &self.pegs[self.get_start_peg(self.config.start_peg_radius)];
        info!("starting peg: {start_peg:?}");
        let mut peg_order = vec![start_peg];
        let mut work_img = self.image.clone();

        let pbar = utils::pbar(self.config.iterations as u64, !self.config.progress_bar)?
            .with_message("Computing blueprint");

        let line_color = 255. * self.config.yarn.opacity;
        let opacity_factor = 1. - self.config.yarn.opacity;

        let mut last_peg = start_peg;
        let mut last_last_peg = last_peg;

        // use a ThreadPool to reduce overhead
        let pool = ThreadPoolBuilder::new().build().unwrap();
        let mut early_stop_count: u32 = 0;

        pool.install(|| {
            'iter: for iter_i in pbar.wrap_iter(0..self.config.iterations) {
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

                if self.early_stop(&mut early_stop_count, min_loss) {
                    info!("Early stopping at iteration {iter_i}");
                    break 'iter;
                }

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

    /// Run a beam search based line pathing algorithm and contruct a [`Blueprint`].
    pub fn compute_beam(&self) -> Result<Blueprint, Box<dyn Error>> {
        if self.line_cache.is_empty() {
            return Err("Line cache is empty, run 'populate_line_cache'.".into());
        };

        let start_peg = self.get_start_peg(self.config.start_peg_radius);
        info!("starting peg: {start_peg:?}");

        let pbar = utils::pbar(self.config.iterations as u64, !self.config.progress_bar)?
            .with_message("Computing blueprint");

        let line_color = 255. * self.config.yarn.opacity;
        let opacity_factor = 1. - self.config.yarn.opacity;

        let mut beam = vec![BeamState {
            peg_order: vec![start_peg],
            loss: 0.,
            image: self.image.clone(),
        }];

        // use a ThreadPool to reduce overhead
        let pool = ThreadPoolBuilder::new().build().unwrap();
        let beam_width_index = self.config.beam_width - 1;
        let mut early_stop_count = 0;

        pool.install(|| {
            'iter: for iter_i in pbar.wrap_iter(0..self.config.iterations) {
                let mut candidates: Vec<_> = beam
                    .par_iter()
                    .flat_map(|beam_state| {
                        let last_peg = &self.pegs[*beam_state.peg_order.last().unwrap()];
                        let last_last_peg = &self.pegs[*beam_state
                            .peg_order
                            .get(beam_state.peg_order.len().saturating_sub(2))
                            .unwrap_or(beam_state.peg_order.last().unwrap())];

                        self.pegs
                            .par_iter()
                            .enumerate()
                            .filter(|(_, peg)| peg.id != last_peg.id && peg.id != last_last_peg.id)
                            .filter_map(|(i, peg)| {
                                let line = self.line_cache.get(&utils::hash_key(last_peg, peg))?;
                                let loss = line.loss(&beam_state.image);
                                Some((loss, i, line, beam_state))
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                // partial sort up to beam width
                candidates.select_nth_unstable_by(beam_width_index, |(loss1, ..), (loss2, ..)| {
                    loss1
                        .partial_cmp(loss2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                let min_loss = candidates
                    .iter()
                    .take(self.config.beam_width)
                    .min_by(|(loss1, ..), (loss2, ..)| {
                        loss1
                            .partial_cmp(loss2)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap()
                    .0;

                if self.early_stop(&mut early_stop_count, min_loss) {
                    info!("Early stopping at iteration {iter_i}");
                    break 'iter;
                }

                beam = candidates
                    .into_iter()
                    .take(self.config.beam_width)
                    .map(|(loss, peg_i, line, beam_state)| {
                        let mut new_image = beam_state.image.clone();
                        line.zip().for_each(|(x, y)| {
                            let pixel = new_image.get_pixel_mut(*x, *y);
                            pixel.0[0] = ((opacity_factor) * pixel.0[0] as f64 + line_color)
                                .round()
                                .min(255.0) as u8;
                        });

                        BeamState {
                            peg_order: beam_state
                                .peg_order
                                .iter()
                                .copied()
                                .chain(std::iter::once(peg_i))
                                .collect(),
                            loss: beam_state.loss + loss,
                            image: new_image,
                        }
                    })
                    .collect();
            }
        });

        let best_state = beam.into_iter().min().unwrap();

        Ok(Blueprint::new(
            best_state
                .peg_order
                .iter()
                .map(|&index| self.pegs[index])
                .collect(),
            self.image.width(),
            self.image.height(),
            Some((255, 255, 255)),
            1.,
        ))
    }

    pub fn compute(&mut self) -> Result<Blueprint, Box<dyn Error>> {
        if self.line_cache.is_empty() {
            warn!("Line cache is empty, populating it.");
            self.populate_line_cache()?;
        }
        if self.config.beam_width > 1 {
            info!("Using beam search algorithm.");
            self.compute_beam()
        } else {
            info!("Using greedy algorithm.");
            self.compute_greedy()
        }
    }
}
