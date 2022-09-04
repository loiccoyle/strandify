use image::{GrayImage, Luma};
use log::debug;
use std::cmp;
use std::iter::zip;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Knitter {
    image: GrayImage,
    pegs: Vec<Peg>,
    yarn: Yarn,
    iterations: i16,
}

impl Knitter {
    pub fn new(image: GrayImage, pegs: Vec<Peg>, yarn: Yarn, iterations: i16) -> Self {
        Self {
            image,
            pegs,
            yarn,
            iterations,
        }
    }

    pub fn from_file(image_path: PathBuf, pegs: Vec<Peg>, yarn: Yarn, iterations: i16) -> Self {
        let image = image::open(image_path).unwrap().into_luma8();
        Self {
            image,
            pegs,
            yarn,
            iterations,
        }
    }

    pub fn knit(&mut self) -> Vec<&Peg> {
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

        let mut min_sum: f64;
        let mut min_peg: Option<&Peg>;
        let mut min_line: Option<(Vec<u32>, Vec<u32>)>;
        let mut last_peg: &Peg;

        for i in 0..self.iterations {
            min_sum = f64::MAX;
            min_peg = None;
            min_line = None;
            last_peg = peg_order.last().unwrap();
            debug!("iteration: {:?}", i);

            for peg in &self.pegs {
                if peg == last_peg {
                    // No need to check the current peg
                    continue;
                }

                let (line_x, line_y) = last_peg.line_to(peg);
                let line_pixels: Vec<&Luma<u8>> = zip(&line_x, &line_y)
                    .map(|(x, y)| self.image.get_pixel(*x, *y))
                    .collect();
                // TODO: check if this overflows
                let pixel_sum: f64 = line_pixels
                    .iter()
                    .fold(0.0, |acc, &pixel| acc + (pixel.0[0] as f64));
                if pixel_sum < min_sum {
                    min_sum = pixel_sum;
                    min_line = Some((line_x, line_y));
                    min_peg = Some(&peg);
                }
            }
            peg_order.push(min_peg.unwrap());
            // TODO: Apply line on image buffer
            // https://docs.rs/image/latest/image/struct.ImageBuffer.html
            let (min_line_x, min_line_y) = min_line.unwrap();
            zip(min_line_x, min_line_y).for_each(|(x, y)| {
                let mut pixel = self.image.get_pixel_mut(x, y);
                pixel.0[0] = (f64::from(pixel.0[0]) * 0.8).round() as u8;
            });
        }
        peg_order
    }
}

#[derive(Debug, Copy, PartialEq)]
pub struct Peg {
    x: u32,
    y: u32,
    id: u16,
}

impl Clone for Peg {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            id: self.id.clone(),
        }
    }
}

impl Peg {
    pub fn new(x: u32, y: u32, id: u16) -> Self {
        Self { x, y, id }
    }
    /// Get the pixel coords connecting 2 pegs.
    pub fn line_to(&self, other_peg: &Peg) -> (Vec<u32>, Vec<u32>) {
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
            return (x_coords.collect(), y_coords);
        } else {
            line_fn = self.line_y_fn_to(other_peg);
            let y_coords = cmp::min(self.y, other_peg.y)..(cmp::max(self.y, other_peg.y) + 1);
            let x_coords: Vec<u32> = y_coords
                .clone()
                .map(line_fn)
                .map(|x| x.round() as u32)
                .collect();
            return (x_coords, y_coords.collect());
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
        let (x_coords, y_coords) = peg_a.line_to(&peg_b);
        assert_eq!(x_coords, vec![0, 1]);
        assert_eq!(y_coords, vec![0, 1]);

        let peg_a = Peg::new(1, 1, 0);
        let peg_b = Peg::new(0, 0, 1);
        let (x_coords, y_coords) = peg_a.line_to(&peg_b);
        assert_eq!(x_coords, vec![0, 1]);
        assert_eq!(y_coords, vec![0, 1]);

        let peg_a = Peg::new(0, 1, 0);
        let peg_b = Peg::new(3, 1, 1);
        let (x_coords, y_coords) = peg_a.line_to(&peg_b);
        assert_eq!(x_coords, vec![0, 1, 2, 3]);
        assert_eq!(y_coords, vec![1, 1, 1, 1]);
    }
}
