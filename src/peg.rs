use std::collections::HashSet;

use itertools::Itertools;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::line::Line;
use crate::utils;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Peg {
    /// Horizontal coordinate of the [`Peg`], (0, 0) is the top left corner of the image.
    pub x: u32,
    /// Vertical coordinate of the [`Peg`], (0, 0) is the top left corner of the image.
    pub y: u32,
    /// [`Peg`] id, should be unique among [`Peg`] instances.
    pub id: u16,
}

impl Peg {
    /// Creates a new [`Peg`].
    pub fn new(x: u32, y: u32, id: u16) -> Self {
        Self { x, y, id }
    }

    /// Get the pixel coords connecting 2 [`Pegs`](Peg) using the Bresenham line algorithm and contruct a [`Line`].
    pub fn line_to(&self, other: &Peg, width: f32) -> Line {
        let mut pixels = HashSet::new();

        // Bresenham's line algorithm
        let dx: i32 = utils::abs_diff(other.x, self.x) as i32;
        let dy: i32 = -(utils::abs_diff(other.y, self.y) as i32);
        let sx: i32 = if self.x < other.x { 1 } else { -1 };
        let sy: i32 = if self.y < other.y { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = self.x as i32;
        let mut y = self.y as i32;

        loop {
            // Add pixels for the current position and its surrounding area based on width
            for ox in -(width as i32 / 2)..=(width as i32 / 2) {
                for oy in -(width as i32 / 2)..=(width as i32 / 2) {
                    pixels.insert((x + ox, y + oy));
                }
            }

            if x == other.x as i32 && y == other.y as i32 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }

        let mut x_coords = Vec::new();
        let mut y_coords = Vec::new();
        pixels
            .iter()
            .sorted_by(|(x_a, y_a), (x_b, y_b)| {
                if x_a + y_a > x_b + y_b {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            })
            .for_each(|(x, y)| {
                x_coords.push(*x as u32);
                y_coords.push(*y as u32);
            });
        Line::new(x_coords, y_coords, self.dist_to(other))
    }

    /// Get the pixels around a [`Peg`] within radius.
    ///
    /// # Arguments
    ///
    /// *`radius`: Pixel radius around [`Peg`].
    pub fn around(&self, radius: u32) -> (Vec<u32>, Vec<u32>) {
        utils::pixels_around((self.x, self.y), radius)
    }

    /// Compute the distance between 2 [`Pegs`](Peg) in pixels.
    pub fn dist_to(&self, other: &Peg) -> u32 {
        let delta_x = utils::abs_diff(self.x, other.x);
        let delta_y = utils::abs_diff(self.y, other.y);
        ((delta_x * delta_x + delta_y * delta_y) as f64).sqrt() as u32
    }

    /// Add 2d jitter to the [`Peg`] returns a new one with added jitter.
    ///
    /// # Arguments
    ///
    /// * `jitter`: Amount of jitter to add, in pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use strandify::peg::Peg;
    /// let peg = Peg::new(10, 10, 0);
    /// let peg_jitter = peg.with_jitter(2);
    /// assert_eq!(peg_jitter.id, peg.id);
    /// ```
    pub fn with_jitter(&self, jitter: i64) -> Self {
        let mut rng = thread_rng();
        Self::new(
            (self.x as i64 + rng.gen_range(-jitter..jitter)) as u32,
            (self.y as i64 + rng.gen_range(-jitter..jitter)) as u32,
            self.id,
        )
    }
}

#[derive(Debug, Clone)]
/// The [`Yarn`] to use when rendering a [`Blueprint`](crate::blueprint::Blueprint).
pub struct Yarn {
    /// Width of the [`Yarn`], in pixels.
    pub width: f32,
    /// [`Yarn`] opacity.
    pub opacity: f64,
    /// [`Yarn`] color.
    pub color: (u8, u8, u8),
}

impl Default for Yarn {
    fn default() -> Self {
        Self {
            width: 1.,
            opacity: 0.2,
            color: (0, 0, 0),
        }
    }
}

impl Yarn {
    /// Creates a new [`Yarn`].
    pub fn new(width: f32, opacity: f64, color: (u8, u8, u8)) -> Self {
        Self {
            width,
            opacity,
            color,
        }
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn peg_line_to() {
        let peg_a = Peg::new(0, 0, 0);
        let peg_b = Peg::new(1, 1, 1);
        let line = peg_a.line_to(&peg_b, 1.);
        assert_eq!(line.x, vec![0, 1]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, f32::sqrt(2.0) as u32);

        let peg_a = Peg::new(1, 1, 0);
        let peg_b = Peg::new(0, 0, 1);
        let line = peg_a.line_to(&peg_b, 1.);
        assert_eq!(line.x, vec![0, 1]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, f32::sqrt(2.0) as u32);

        // horizontal line
        let peg_a = Peg::new(0, 1, 0);
        let peg_b = Peg::new(3, 1, 1);
        let line = peg_a.line_to(&peg_b, 1.);
        assert_eq!(line.x, vec![0, 1, 2, 3]);
        assert_eq!(line.y, vec![1, 1, 1, 1]);
        assert_eq!(line.dist, 3);

        // vertical line
        let peg_a = Peg::new(0, 0, 0);
        let peg_b = Peg::new(0, 1, 1);
        let line = peg_a.line_to(&peg_b, 1.);
        assert_eq!(line.x, vec![0, 0]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, 1);
        assert_eq!(line.zip().len(), 2);
    }

    #[test]
    fn peg_around() {
        let peg = Peg::new(10, 10, 0);
        let (x_coords, y_coords) = peg.around(1);
        assert_eq!(x_coords, vec![9, 10, 10, 10, 11]);
        assert_eq!(y_coords, vec![10, 9, 10, 11, 10]);
    }

    #[test]
    fn peg_jitter() {
        let peg = Peg::new(10, 10, 0);
        let jitter = 2;
        let peg_jitter = peg.with_jitter(jitter);
        assert!(peg_jitter.x <= (peg.x as i64 + jitter) as u32);
        assert!(peg_jitter.x >= (peg.x as i64 - jitter) as u32);
        assert_eq!(peg_jitter.id, peg.id);
    }
}
