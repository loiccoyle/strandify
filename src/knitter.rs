use image::{GrayImage, Luma};
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

    pub fn knit(&self) -> Vec<&Peg> {
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

        let mut min: f64;
        let mut min_peg: Option<&Peg>;
        let mut last_peg: &Peg;

        for _ in 0..self.iterations {
            min = f64::MAX;
            min_peg = None;

            for peg in &self.pegs {
                last_peg = peg_order.last().unwrap();
                if peg == last_peg {
                    // No need to check the current peg
                    continue;
                }

                let (line_x, line_y) = last_peg.line_to(peg, &self.yarn);
                let line_pixels: Vec<u8> = zip(line_x, line_y)
                    .map(|(x, y)| self.image[(x, y)].0[0])
                    .collect();
                let pixel_sum: f64 = line_pixels.iter().sum::<u8>() as f64;
                if pixel_sum < min {
                    min = pixel_sum;
                    min_peg = Some(&peg);
                }
            }
            peg_order.push(min_peg.unwrap());
            // TODO: Apply line on image buffer
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
    pub fn line_to(&self, other_peg: &Peg, yarn: &Yarn) -> (Vec<u32>, Vec<u32>) {
        unimplemented!()
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
