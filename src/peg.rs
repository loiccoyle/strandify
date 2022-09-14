use std::cmp;
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
    pub fn line_to(&self, other_peg: &Peg) -> Line {
        let delta_x = utils::abs_diff(self.x, other_peg.x);
        let delta_y = utils::abs_diff(self.y, other_peg.y);

        // vertical line
        if delta_x == 0 && delta_y != 0 {
            let y_coords = cmp::min(self.y, other_peg.y)..(cmp::max(self.y, other_peg.y) + 1);
            return Line::new(vec![self.x; y_coords.len()], y_coords.collect());
        }

        let line_fn;
        if delta_x >= delta_y {
            line_fn = self.line_x_fn_to(other_peg);
            let x_coords = cmp::min(self.x, other_peg.x)..(cmp::max(self.x, other_peg.x) + 1);
            let y_coords: Vec<u32> = x_coords
                .clone()
                .map(line_fn)
                .map(|y| y.round() as u32)
                .collect();
            return Line::new(x_coords.collect(), y_coords);
        } else {
            line_fn = self.line_y_fn_to(other_peg);
            let y_coords = cmp::min(self.y, other_peg.y)..(cmp::max(self.y, other_peg.y) + 1);
            let x_coords: Vec<u32> = y_coords
                .clone()
                .map(line_fn)
                .map(|x| x.round() as u32)
                .collect();
            return Line::new(x_coords, y_coords.collect());
        }
    }

    fn get_line_coefs(&self, other_peg: &Peg) -> (f64, f64) {
        let slope: f64 = (f64::from(other_peg.y) - f64::from(self.y))
            / (f64::from(other_peg.x) - f64::from(self.x));
        let intercept: f64 = f64::from(self.y) - slope * f64::from(self.x);
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

    /// Get the pixels around a peg within radius
    ///
    /// # Arguments
    ///
    /// *`radius`- pixel radius around peg
    pub fn around(&self, radius: u32) -> (Vec<u32>, Vec<u32>) {
        utils::pixels_around((self.x, self.y), radius)
    }
}

#[derive(Debug)]
pub struct Line {
    pub x: Vec<u32>,
    pub y: Vec<u32>,
    pub dist: u32,
}

impl Line {
    pub fn new(x: Vec<u32>, y: Vec<u32>) -> Self {
        let delta_x = utils::abs_diff(*x.first().unwrap(), *x.last().unwrap());
        let delta_y = utils::abs_diff(*y.first().unwrap(), *y.last().unwrap());
        Self {
            x,
            y,
            dist: ((delta_x * delta_x + delta_y * delta_y) as f64)
                .sqrt()
                .round() as u32,
        }
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
            peg_order: peg_order.into_iter().map(|a| a.clone()).collect(),
        }
    }

    pub fn to_file(&self, file_path: PathBuf) {
        let mut file = File::create(file_path).unwrap();

        self.peg_order
            .iter()
            .for_each(|peg| write!(file, "{}\n", peg.id).unwrap());
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
    fn yarn_delta() {
        let yarn = Yarn::new(1, 0.5);
        assert_eq!(yarn.delta(), 128);
    }
}
