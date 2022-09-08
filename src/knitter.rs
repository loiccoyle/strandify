use image::GrayImage;
use log::debug;
use std::iter::zip;
use std::path::PathBuf;
use std::{cmp, collections::HashMap};

use indicatif::ProgressIterator;

use crate::utils;

const LIGHTEN_FACTOR: f64 = 1.05;
const DARKEN_FACTOR: f64 = 0.95;

#[derive(Debug)]
pub struct Knitter {
    pub image: GrayImage,
    pub pegs: Vec<Peg>,
    pub yarn: Yarn,
    pub iterations: u16,
    pub line_cache: HashMap<(u16, u16), Line>,
}

impl Knitter {
    pub fn new(image: GrayImage, pegs: Vec<Peg>, yarn: Yarn, iterations: u16) -> Self {
        let line_cache = HashMap::new();
        let mut out = Self {
            image,
            pegs,
            yarn,
            iterations,
            line_cache,
        };
        out.populate_line_cache();
        return out;
    }

    pub fn from_file(image_path: PathBuf, pegs: Vec<Peg>, yarn: Yarn, iterations: u16) -> Self {
        let image = image::open(image_path).unwrap().into_luma8();
        let line_cache = HashMap::new();
        let mut out = Self {
            image,
            pegs,
            yarn,
            iterations,
            line_cache,
        };
        out.populate_line_cache();
        return out;
    }

    pub fn populate_line_cache(&mut self) {
        for peg_a in self
            .pegs
            .iter()
            .progress()
            .with_message("Populating hash map")
            .with_style(utils::progress_style())
        {
            for peg_b in self.pegs.iter() {
                if peg_a.id == peg_b.id {
                    continue;
                }
                self.line_cache
                    .insert(self.hash_key(peg_a, peg_b), peg_a.line_to(peg_b));
            }
        }
    }

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

        let mut peg_order = vec![&self.pegs[0]];
        let mut work_img = self.image.clone();

        let mut min_loss: f64;
        let mut min_peg: Option<&Peg>;
        let mut min_line: Option<&Line>;
        let mut last_peg: &Peg;

        let mut line_coords;
        let mut loss: f64;

        for _ in (0..self.iterations)
            .progress()
            .with_message("Computing peg order")
            .with_style(utils::progress_style())
        {
            min_loss = f64::MAX;
            min_peg = None;
            min_line = None;
            last_peg = peg_order.last().unwrap();

            for peg in &self.pegs {
                if peg.id == last_peg.id {
                    // No need to check the current peg
                    continue;
                }

                line_coords = self.line_cache.get(&self.hash_key(last_peg, peg)).unwrap();
                loss = line_coords
                    .zip()
                    .map(|(x, y)| work_img.get_pixel(*x, *y))
                    .fold(0.0, |acc, &pixel| acc + (pixel.0[0] as f64))
                    / line_coords.len() as f64;
                if loss < min_loss {
                    min_loss = loss;
                    min_line = Some(line_coords.clone());
                    min_peg = Some(&peg);
                }
            }
            peg_order.push(min_peg.unwrap());
            // https://docs.rs/image/latest/image/struct.ImageBuffer.html
            min_line.unwrap().zip().for_each(|(x, y)| {
                let mut pixel = work_img.get_pixel_mut(*x, *y);
                // TODO: check lighten factor
                pixel.0[0] = (pixel.0[0] as f64 * LIGHTEN_FACTOR).ceil() as u8;
                // pixel.0[0] = cmp::max(pixel.0[0] as u16 + 1, 255) as u8;
            });
        }
        work_img.save("work_img.png").unwrap();
        peg_order
    }

    fn hash_key(&self, peg_a: &Peg, peg_b: &Peg) -> (u16, u16) {
        if peg_a.id < peg_b.id {
            return (peg_a.id, peg_b.id);
        } else {
            return (peg_b.id, peg_a.id);
        }
    }

    pub fn knit(&self, peg_order: &Vec<&Peg>) -> image::GrayImage {
        // Create white img
        let mut img = image::GrayImage::new(self.image.width(), self.image.height());
        for (_, _, pixel) in img.enumerate_pixels_mut() {
            pixel.0[0] = 255;
        }

        for (peg_a, peg_b) in peg_order.iter().zip(peg_order.iter().skip(1)) {
            peg_a.line_to(peg_b).zip().for_each(|(x, y)| {
                let mut pixel = img.get_pixel_mut(*x, *y);
                pixel.0[0] = (pixel.0[0] as f64 * DARKEN_FACTOR).floor() as u8;
            })
        }
        img
    }
}

#[derive(Debug)]
pub struct Line {
    pub x: Vec<u32>,
    pub y: Vec<u32>,
}

impl Line {
    pub fn len(&self) -> usize {
        self.x.len()
    }

    pub fn zip(&self) -> std::iter::Zip<std::slice::Iter<u32>, std::slice::Iter<u32>> {
        zip(&self.x, &self.y)
    }
}

#[derive(Debug)]
pub struct Peg {
    x: u32,
    y: u32,
    id: u16,
}

impl Peg {
    pub fn new(x: u32, y: u32, id: u16) -> Self {
        Self { x, y, id }
    }
    /// Get the pixel coords connecting 2 pegs.
    pub fn line_to(&self, other_peg: &Peg) -> Line {
        let delta_x: i64 = (i64::from(self.x) - i64::from(other_peg.x)).abs();
        let delta_y: i64 = (i64::from(self.y) - i64::from(other_peg.y)).abs();

        let line_fn;
        if delta_x >= delta_y {
            line_fn = self.line_x_fn_to(other_peg);
            let x_coords = cmp::min(self.x, other_peg.x)..(cmp::max(self.x, other_peg.x) + 1);
            let y_coords: Vec<u32> = x_coords
                .clone()
                .map(line_fn)
                .map(|y| y.round() as u32)
                .collect();
            return Line {
                x: x_coords.collect(),
                y: y_coords,
            };
        } else {
            line_fn = self.line_y_fn_to(other_peg);
            let y_coords = cmp::min(self.y, other_peg.y)..(cmp::max(self.y, other_peg.y) + 1);
            let x_coords: Vec<u32> = y_coords
                .clone()
                .map(line_fn)
                .map(|x| x.round() as u32)
                .collect();
            return Line {
                x: x_coords,
                y: y_coords.collect(),
            };
        }
    }

    fn get_line_coefs(&self, other_peg: &Peg) -> (f64, f64) {
        let slope: f64 = (f64::from(other_peg.y) - f64::from(self.y))
            / (f64::from(other_peg.x) - f64::from(self.x));
        let intercept: f64 = f64::from(self.y) - slope * f64::from(self.x);
        // TODO: how to handle the case where the line is vertical
        (slope, intercept)
    }

    fn line_x_fn_to(&self, other_peg: &Peg) -> Box<dyn FnMut(u32) -> f64> {
        let (slope, intercept) = self.get_line_coefs(other_peg);
        Box::new(move |x| slope * f64::from(x) + intercept)
    }

    fn line_y_fn_to(&self, other_peg: &Peg) -> Box<dyn FnMut(u32) -> f64> {
        let (slope, intercept) = self.get_line_coefs(other_peg);
        Box::new(move |y| f64::from(y) / slope - intercept / slope)
    }
}

#[derive(Debug)]
pub struct Yarn {
    width: u8,
    opacity: f32,
}

impl Yarn {
    pub fn new(width: u8, opacity: f32) -> Self {
        Self { width, opacity }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_line_coefs() {
        let peg_a = Peg::new(0, 0, 0);
        let peg_b = Peg::new(1, 1, 1);
        let (slope, intercept) = peg_a.get_line_coefs(&peg_b);
        assert_eq!(slope, 1.);
        assert_eq!(intercept, 0.);

        let peg_a = Peg::new(1, 1, 0);
        let peg_b = Peg::new(0, 1, 1);
        let (slope, intercept) = peg_a.get_line_coefs(&peg_b);
        assert_eq!(slope, 0.);
        assert_eq!(intercept, 1.);
    }

    #[test]
    fn test_line_to() {
        let peg_a = Peg::new(0, 0, 0);
        let peg_b = Peg::new(1, 1, 1);
        let line = peg_a.line_to(&peg_b);
        assert_eq!(line.x, vec![0, 1]);
        assert_eq!(line.y, vec![0, 1]);

        let peg_a = Peg::new(1, 1, 0);
        let peg_b = Peg::new(0, 0, 1);
        let line = peg_a.line_to(&peg_b);
        assert_eq!(line.x, vec![0, 1]);
        assert_eq!(line.y, vec![0, 1]);

        let peg_a = Peg::new(0, 1, 0);
        let peg_b = Peg::new(3, 1, 1);
        let line = peg_a.line_to(&peg_b);
        assert_eq!(line.x, vec![0, 1, 2, 3]);
        assert_eq!(line.y, vec![1, 1, 1, 1]);
    }
}
