use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::path::PathBuf;

use crate::utils;

#[derive(Debug, Clone, Copy)]
pub struct Peg {
    pub x: u32,
    pub y: u32,
    pub id: u16,
}

impl Peg {
    pub fn new(x: u32, y: u32, id: u16) -> Self {
        Self { x, y, id }
    }

    /// Get the pixel coords connecting 2 pegs.
    pub fn line_to(&self, other: &Peg) -> Line {
        let delta_x = utils::abs_diff(self.x, other.x);
        let delta_y = utils::abs_diff(self.y, other.y);
        let dist = self.dist_to(other);

        // vertical line
        let x_coords: Vec<u32>;
        let y_coords: Vec<u32>;

        if delta_x == 0 && delta_y != 0 {
            y_coords = (cmp::min(self.y, other.y)..(cmp::max(self.y, other.y) + 1)).collect();
            x_coords = vec![self.x; y_coords.len()]
            // return Line::new(x_coords, y_coords, dist);
        } else if delta_x >= delta_y {
            let line_fn = self.line_x_fn_to(other);
            x_coords = (cmp::min(self.x, other.x)..(cmp::max(self.x, other.x) + 1)).collect();
            y_coords = x_coords
                .clone()
                .into_iter()
                .map(line_fn)
                .map(|y| y.round() as u32)
                .collect();
        } else {
            let line_fn = self.line_y_fn_to(other);
            y_coords = (cmp::min(self.y, other.y)..(cmp::max(self.y, other.y) + 1)).collect();
            x_coords = y_coords
                .clone()
                .into_iter()
                .map(line_fn)
                .map(|x| x.round() as u32)
                .collect();
        }
        Line::new(x_coords, y_coords, dist)
    }

    fn get_line_coefs(&self, other: &Peg) -> (f64, f64) {
        let slope: f64 =
            (f64::from(other.y) - f64::from(self.y)) / (f64::from(other.x) - f64::from(self.x));
        let intercept: f64 = f64::from(self.y) - slope * f64::from(self.x);
        (slope, intercept)
    }

    fn line_x_fn_to(&self, other: &Peg) -> Box<dyn FnMut(u32) -> f64> {
        let (slope, intercept) = self.get_line_coefs(other);
        Box::new(move |x| slope * f64::from(x) + intercept)
    }

    fn line_y_fn_to(&self, other: &Peg) -> Box<dyn FnMut(u32) -> f64> {
        let (slope, intercept) = self.get_line_coefs(other);
        Box::new(move |y| f64::from(y) / slope - intercept / slope)
    }

    /// Get the pixels around a peg within radius
    ///
    /// # Arguments
    ///
    /// *`radius`- pixel radius around peg
    pub fn around(&self, radius: u32) -> (Vec<u32>, Vec<u32>) {
        utils::pixels_around((self.x, self.y), radius)
    }

    /// Compute the distance between 2 pegs
    pub fn dist_to(&self, other: &Peg) -> u32 {
        let delta_x = utils::abs_diff(self.x, other.x);
        let delta_y = utils::abs_diff(self.y, other.y);
        ((delta_x * delta_x + delta_y * delta_y) as f64).sqrt() as u32
    }
}

#[derive(Debug)]
pub struct Line {
    pub x: Vec<u32>,
    pub y: Vec<u32>,
    pub dist: u32,
}

impl Line {
    pub fn new(x: Vec<u32>, y: Vec<u32>, dist: u32) -> Self {
        assert_eq!(x.len(), y.len(), "`x` and `y` should have the same length");
        Self { x, y, dist }
    }

    /// Construct a new Line with a width
    pub fn with_width(&self, width: u32) -> Self {
        let radius = width / 2;
        let mut pixels: HashSet<(u32, u32)> = HashSet::new();
        for (x, y) in self.zip() {
            let (around_x, around_y) = utils::pixels_around((*x, *y), radius);
            for pixel in around_x.into_iter().zip(around_y) {
                pixels.insert(pixel);
            }
        }

        let mut x_coords = vec![];
        let mut y_coords = vec![];
        for (x_pixel, y_pixel) in pixels {
            x_coords.push(x_pixel);
            y_coords.push(y_pixel);
        }

        Self::new(x_coords, y_coords, self.dist)
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }

    pub fn zip(&self) -> std::iter::Zip<std::slice::Iter<u32>, std::slice::Iter<u32>> {
        zip(&self.x, &self.y)
    }
}

#[derive(Debug)]
pub struct Yarn {
    //TODO: add width to line computations
    width: u8,
    opacity: f32,
}

impl Yarn {
    pub fn new(width: u8, opacity: f32) -> Self {
        Self { width, opacity }
    }

    pub fn delta(&self) -> u8 {
        (self.opacity * 255.).round() as u8
    }
}

#[derive(Debug)]
pub struct Blueprint {
    pub peg_order: Vec<Peg>,
}

impl Blueprint {
    pub fn new(peg_order: Vec<Peg>) -> Self {
        Self { peg_order }
    }

    pub fn from_refs(peg_order: Vec<&Peg>) -> Self {
        Self {
            peg_order: peg_order.into_iter().copied().collect(),
        }
    }

    pub fn to_file(&self, file_path: PathBuf) {
        let mut file = File::create(file_path).unwrap();

        self.peg_order
            .iter()
            .for_each(|peg| writeln!(file, "{}", peg.id).unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peg_get_line_coefs() {
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
    fn test_peg_line_to() {
        let peg_a = Peg::new(0, 0, 0);
        let peg_b = Peg::new(1, 1, 1);
        let line = peg_a.line_to(&peg_b);
        assert_eq!(line.x, vec![0, 1]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, f32::from(2.0).sqrt() as u32);

        let peg_a = Peg::new(1, 1, 0);
        let peg_b = Peg::new(0, 0, 1);
        let line = peg_a.line_to(&peg_b);
        assert_eq!(line.x, vec![0, 1]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, f32::from(2.0).sqrt() as u32);

        // horizontal line
        let peg_a = Peg::new(0, 1, 0);
        let peg_b = Peg::new(3, 1, 1);
        let line = peg_a.line_to(&peg_b);
        assert_eq!(line.x, vec![0, 1, 2, 3]);
        assert_eq!(line.y, vec![1, 1, 1, 1]);
        assert_eq!(line.dist, 3);

        // vertical line
        let peg_a = Peg::new(0, 0, 0);
        let peg_b = Peg::new(0, 1, 1);
        let line = peg_a.line_to(&peg_b);
        assert_eq!(line.x, vec![0, 0]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, 1);
    }

    #[test]
    fn test_peg_around() {
        let peg = Peg::new(10, 10, 0);
        let (x_coords, y_coords) = peg.around(1);
        assert_eq!(x_coords, vec![9, 10, 10, 10, 11]);
        assert_eq!(y_coords, vec![10, 9, 10, 11, 10]);
    }

    #[test]
    fn line_with_width() {
        let line = Line::new(vec![10, 10], vec![10, 11], 1);
        let mut line_wide = line.with_width(2);
        line_wide.x.sort();
        line_wide.y.sort();
        assert_eq!(line_wide.x, vec![9, 9, 10, 10, 10, 10, 11, 11]);
        assert_eq!(line_wide.y, vec![9, 10, 10, 10, 11, 11, 11, 12]);
    }

    #[test]
    fn yarn_delta() {
        let yarn = Yarn::new(1, 0.5);
        assert_eq!(yarn.delta(), 128);
    }
}
