use log::warn;
use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use image::GrayImage;
#[cfg(feature = "parallel")]
use indicatif::ParallelProgressIterator;
#[cfg(not(feature = "parallel"))]
use indicatif::ProgressIterator;
use itertools::Itertools;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[cfg(feature = "parallel")]
use rayon::ThreadPoolBuilder;

use crate::blueprint::Blueprint;
use crate::line::Line;
use crate::peg::{Peg, Yarn};
use crate::utils;

#[derive(Debug, Clone)]
/// Pathing algorithm early stopping configuration.
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
/// Pathing algorithm configuration.
pub struct PatherConfig {
    /// Number of [`Peg`] connections.
    pub iterations: usize,
    /// The [`Yarn`] to use when computing the path. When using a high [`Yarn::opacity`] the lines
    /// will overlap less.
    pub yarn: Yarn,
    /// [`EarlyStopConfig`].
    pub early_stop: EarlyStopConfig,
    /// Radius around [`Pegs`](Peg), in pixels, to use to determine the starting [`Peg`].
    pub start_peg_radius: u32,
    /// Don't connect [`Pegs`](Peg) within distance, in pixels.
    pub skip_peg_within: u32,
    /// Beam search width, larger values will be lead to more accurate paths at the expense of
    /// compute time.
    pub beam_width: usize,
    /// Display progress bar.
    pub progress_bar: bool,
}

impl PatherConfig {
    /// Creates a new [`PatherConfig`].
    pub fn new(
        iterations: usize,
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
    /// Array of [`Pegs`](Peg) to use to compute the path.
    pub pegs: Vec<Peg>,
    /// Pathing algorithm configuration.
    pub config: PatherConfig,
    /// Holds the pixel coords of all the lines, run [Pather::populate_line_cache] to populate the
    /// cache.
    pub line_cache: HashMap<(usize, usize), Line>,
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

    /// Populate the [Pather::line_cache] with the pixel coords of all the lines between the [`Peg`] pairs.
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

        let key_line_pixels = utils::iter_or_par_iter!(peg_combinations)
            .progress_with(pbar)
            .map(|(peg_a, peg_b)| {
                (
                    utils::hash_key(peg_a, peg_b),
                    peg_a.line_to(peg_b, self.config.yarn.width as i32),
                )
            })
            .collect::<Vec<((usize, usize), Line)>>();

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

    /// Run a greedy line pathing algorithm and construct a [`Blueprint`].
    pub fn compute_greedy(&self) -> Result<Blueprint, Box<dyn Error>> {
        if self.line_cache.is_empty() {
            return Err("Line cache is empty, run 'populate_line_cache'.".into());
        };

        let pbar = utils::pbar(self.config.iterations as u64, !self.config.progress_bar)?
            .with_message("Computing blueprint");

        let line_color = 255. * self.config.yarn.opacity;

        let compute = || {
            let start_peg = &self.pegs[self.get_start_peg(self.config.start_peg_radius)];
            debug!("Starting peg: {start_peg:?}");
            let mut peg_order = vec![start_peg];
            let mut work_img = self.image.clone();
            let mut last_peg = start_peg;
            let mut last_last_peg = last_peg;
            let mut early_stop_count: u32 = 0;

            'iter: for iter_i in pbar.wrap_iter(0..self.config.iterations) {
                let (min_loss, min_peg, min_line) = utils::iter_or_par_iter!(self.pegs)
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

                min_line.draw(&mut work_img, self.config.yarn.opacity, line_color);
            }
            peg_order
        };

        let order;
        #[cfg(feature = "parallel")]
        {
            let pool = ThreadPoolBuilder::new().build()?;
            order = pool.install(compute);
        }

        #[cfg(not(feature = "parallel"))]
        {
            order = compute();
        }

        Ok(Blueprint::from_refs(
            order,
            self.image.width(),
            self.image.height(),
            Some((255, 255, 255)),
            1.,
            self.config.progress_bar,
        ))
    }

    /// Run a beam search based line pathing algorithm and construct a [`Blueprint`].
    pub fn compute_beam(&self) -> Result<Blueprint, Box<dyn Error>> {
        if self.line_cache.is_empty() {
            return Err("Line cache is empty, run 'populate_line_cache'.".into());
        };

        let start_peg = self.get_start_peg(self.config.start_peg_radius);
        debug!("Starting peg: {start_peg:?}");

        let pbar = utils::pbar(self.config.iterations as u64, !self.config.progress_bar)?
            .with_message("Computing blueprint");

        let line_color = 255. * self.config.yarn.opacity;

        let mut beam = vec![BeamState {
            peg_order: vec![start_peg],
            loss: 0.,
            image: self.image.clone(),
        }];

        // use a ThreadPool to reduce overhead
        let beam_width_index = self.config.beam_width - 1;

        let compute = |beam: &mut Vec<BeamState>| {
            let mut early_stop_count = 0;

            'iter: for iter_i in pbar.wrap_iter(0..self.config.iterations) {
                let mut candidates: Vec<_> = utils::iter_or_par_iter!(beam)
                    .flat_map(|beam_state| {
                        let last_peg = &self.pegs[*beam_state.peg_order.last().unwrap()];
                        let last_last_peg = &self.pegs[*beam_state
                            .peg_order
                            .get(beam_state.peg_order.len().saturating_sub(2))
                            .unwrap_or(beam_state.peg_order.last().unwrap())];

                        utils::iter_or_par_iter!(self.pegs)
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

                *beam = candidates
                    .into_iter()
                    .take(self.config.beam_width)
                    .map(|(loss, peg_i, line, beam_state)| {
                        let mut new_img = beam_state.image.clone();
                        line.draw(&mut new_img, self.config.yarn.opacity, line_color);

                        BeamState {
                            peg_order: beam_state
                                .peg_order
                                .iter()
                                .copied()
                                .chain(std::iter::once(peg_i))
                                .collect(),
                            loss: beam_state.loss + loss,
                            image: new_img,
                        }
                    })
                    .collect();
            }
        };

        #[cfg(feature = "parallel")]
        {
            let pool = ThreadPoolBuilder::new().build()?;
            pool.install(|| compute(&mut beam));
        }

        #[cfg(not(feature = "parallel"))]
        {
            compute(&mut beam);
        }

        let best_state = beam.into_iter().min().ok_or("Beam search failed.")?;

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
            self.config.progress_bar,
        ))
    }

    /// Run the pathing algorithm. Will use the [greedy](Pather::compute_greedy) algorithm when
    /// [`PatherConfig::beam_width`] equals 1 and the [beam search](Pather::compute_beam) algorithm
    /// otherwise.
    ///
    /// If [`Pather::line_cache`] is empty, will [populate](Pather::populate_line_cache) it.
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
