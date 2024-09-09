use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::line::Line;
use crate::utils;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
/// The [`Peg`] around which the [`Yarn`] is weaved.
pub struct Peg {
    /// Horizontal coordinate of the [`Peg`], (0, 0) is the top left corner of the image.
    pub x: u32,
    /// Vertical coordinate of the [`Peg`], (0, 0) is the top left corner of the image.
    pub y: u32,
    /// [`Peg`] id, should be unique among [`Peg`] instances.
    pub id: usize,
}

impl Peg {
    /// Creates a new [`Peg`].
    pub fn new(x: u32, y: u32) -> Self {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        Self { x, y, id }
    }

    /// Get the pixel coords connecting 2 [`Pegs`](Peg) using the Bresenham line algorithm and contruct a [`Line`].
    ///
    /// # Arguments:
    ///
    /// * `other`: the other [`Peg`] to draw the line to.
    /// * `width`: the width of the line. The line resulting line width can only be odd, which
    ///     leads to unintuitive behaviours:
    ///     * `width=0` -> 1 pixel wide
    ///     * `width=1` -> 1 pixel wide
    ///     * `width=2` -> 3 pixels wide
    ///     * `width=3` -> 3 pixels wide
    ///     * `width=4` -> 5 pixels wide
    ///     * and so on
    /// * `min_max`: min and max values of the line (x_min, x_max, y_min, y_max), used to crop the line to the image bounds.
    pub fn line_to(&self, other: &Peg, width: u32, min_max: Option<(u32, u32, u32, u32)>) -> Line {
        let mut pixels = HashSet::new();
        let half_width = width as i32 / 2;
        let (x_min, x_max, y_min, y_max): (i32, i32, i32, i32) = match min_max {
            Some((x_min, x_max, y_min, y_max)) => {
                (x_min as i32, x_max as i32, y_min as i32, y_max as i32)
            }
            None => (0, i32::MAX, 0, i32::MAX),
        };

        // Bresenham's line algorithm
        let dx = self.x.abs_diff(other.x) as i32;
        let dy = self.y.abs_diff(other.y) as i32;
        let sx = if self.x < other.x { 1 } else { -1 };
        let sy = if self.y < other.y { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = self.x as i32;
        let mut y = self.y as i32;

        // Determine the number of steps (the maximum of dx or dy)
        let steps = dx.max(dy);

        // Iterate through the number of steps to draw the line
        for _ in 0..=steps {
            // Add pixels for the current position and its surrounding area based on width
            for ox in -(half_width)..=(half_width) {
                for oy in -(half_width)..=(half_width) {
                    pixels.insert(((x + ox).clamp(x_min, x_max), (y + oy).clamp(y_min, y_max)));
                }
            }
            if x == other.x as i32 && y == other.y as i32 {
                break;
            }
            let e2 = 2 * err;
            // Move in the x-direction
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            // Move in the y-direction
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }

        // Convert HashSet of pixels to vectors of x and y coordinates
        let (x_vec, y_vec): (Vec<u32>, Vec<u32>) = pixels
            .into_iter()
            .map(|(x, y)| (x as u32, y as u32))
            .unzip();
        Line::new(x_vec, y_vec, self.dist_to(other))
    }

    /// Get the pixels around a [`Peg`] within radius.
    ///
    /// # Arguments
    ///
    /// * `radius`: Pixel radius around [`Peg`].
    pub fn around(&self, radius: u32) -> (Vec<u32>, Vec<u32>) {
        utils::pixels_around((self.x, self.y), radius)
    }

    /// Compute the distance between 2 [`Pegs`](Peg) in pixels.
    pub fn dist_to(&self, other: &Peg) -> u32 {
        let delta_x = utils::abs_diff(self.x, other.x);
        let delta_y = utils::abs_diff(self.y, other.y);
        ((delta_x * delta_x + delta_y * delta_y) as f64)
            .sqrt()
            .round() as u32
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
    /// let peg = Peg::new(10, 10);
    /// let peg_jitter = peg.with_jitter(2);
    /// assert_eq!(peg_jitter.id, peg.id);
    /// ```
    pub fn with_jitter(&self, jitter: i64) -> Self {
        let mut rng = thread_rng();
        Self {
            x: (self.x as i64 + rng.gen_range(-jitter..jitter)) as u32,
            y: (self.y as i64 + rng.gen_range(-jitter..jitter)) as u32,
            id: self.id,
        }
    }
}

/// Helper functions to generate [`Pegs`](Peg) based on different shapes.
pub mod shape {
    use super::*;

    fn coords_to_pegs(coords: (Vec<u32>, Vec<u32>)) -> Vec<Peg> {
        coords
            .0
            .into_iter()
            .zip(coords.1)
            .map(|(x, y)| Peg::new(x, y))
            .collect()
    }

    /// Generate [`Pegs`](Peg) in a square.
    ///
    /// # Arguments
    ///
    /// * `top_left`: Top left corner of the square.
    /// * `length`: Length of the side of the square.
    /// * `n_pegs`: Number of pegs.
    pub fn square(top_left: (u32, u32), length: u32, n_pegs: usize) -> Vec<Peg> {
        coords_to_pegs(utils::square_coords(top_left, length, n_pegs))
    }

    /// Generate [`Pegs`](Peg) in a rectangle.
    ///
    /// # Arguments
    ///
    /// * `top_left`: Top left corner of the square.
    /// * `width`: Width of the rectangle.
    /// * `height`: height of the rectangle.
    /// * `n_pegs`: Number of pegs.
    pub fn rectangle(top_left: (u32, u32), width: u32, height: u32, n_pegs: usize) -> Vec<Peg> {
        coords_to_pegs(utils::rectangle_coords(top_left, width, height, n_pegs))
    }

    /// Generate [`Pegs`](Peg) in a circle.
    ///
    /// # Arguments
    ///
    /// * `center`: The center of the circle.
    /// * `radius`: Radius of the circle.
    /// * `n_pegs`: Number of pegs.
    pub fn circle(center: (u32, u32), radius: u32, n_pegs: usize) -> Vec<Peg> {
        coords_to_pegs(utils::circle_coords(center, radius, n_pegs))
    }

    /// Generate [`Pegs`](Peg) on a line.
    ///
    /// # Arguments
    ///
    /// * `start`: Start point of the line.
    /// * `end`: End point of the line.
    /// * `n_pegs`: Number of pegs.
    pub fn line(start: (u32, u32), end: (u32, u32), n_pegs: usize) -> Vec<Peg> {
        coords_to_pegs(utils::line_coords(start, end, n_pegs))
    }
}

#[derive(Debug, Clone)]
/// A [`Yarn`], used to control how to render a [`Blueprint`](crate::blueprint::Blueprint) and to
/// influence the [`Pather`](crate::pather::Pather)'s pathing algorithm.
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
        let peg_a = Peg::new(0, 0);
        let peg_b = Peg::new(1, 1);
        let mut line = peg_a.line_to(&peg_b, 1, None);
        line.x.sort();
        line.y.sort();

        assert_eq!(line.x, vec![0, 1]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, f32::sqrt(2.0) as u32);

        let peg_a = Peg::new(1, 1);
        let peg_b = Peg::new(0, 0);
        let mut line = peg_a.line_to(&peg_b, 1, None);
        line.x.sort();
        line.y.sort();
        assert_eq!(line.x, vec![0, 1]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, f32::sqrt(2.0) as u32);

        // horizontal line
        let peg_a = Peg::new(0, 1);
        let peg_b = Peg::new(3, 1);
        let mut line = peg_a.line_to(&peg_b, 1, None);
        line.x.sort();
        line.y.sort();
        assert_eq!(line.x, vec![0, 1, 2, 3]);
        assert_eq!(line.y, vec![1, 1, 1, 1]);
        assert_eq!(line.dist, 3);

        // vertical line
        let peg_a = Peg::new(0, 0);
        let peg_b = Peg::new(0, 1);
        let mut line = peg_a.line_to(&peg_b, 1, None);
        line.x.sort();
        line.y.sort();
        assert_eq!(line.x, vec![0, 0]);
        assert_eq!(line.y, vec![0, 1]);
        assert_eq!(line.dist, 1);
        assert_eq!(line.zip().len(), 2);
    }

    #[test]
    fn peg_line_to_width() {
        let peg_a = Peg::new(5, 5);
        let peg_b = Peg::new(5, 5);
        let line = peg_a.line_to(&peg_b, 0, None);
        assert_eq!(line.x, vec![5]);
        assert_eq!(line.y, vec![5]);
        let line = peg_a.line_to(&peg_b, 1, None);
        assert_eq!(line.x, vec![5]);
        assert_eq!(line.y, vec![5]);
        assert_eq!(line.dist, 0);
        let line = peg_a.line_to(&peg_b, 2, None);
        // we go +1 in all directions
        assert_eq!(*line.x.iter().max().unwrap(), 6);
        assert_eq!(*line.x.iter().min().unwrap(), 4);
        assert_eq!(*line.y.iter().max().unwrap(), 6);
        assert_eq!(*line.y.iter().min().unwrap(), 4);
        assert_eq!(line.dist, 0);
        let line = peg_a.line_to(&peg_b, 3, None);
        // we go +1 in all directions
        assert_eq!(*line.x.iter().max().unwrap(), 6);
        assert_eq!(*line.x.iter().min().unwrap(), 4);
        assert_eq!(*line.y.iter().max().unwrap(), 6);
        assert_eq!(*line.y.iter().min().unwrap(), 4);
        assert_eq!(line.dist, 0);
        let line = peg_a.line_to(&peg_b, 4, None);
        // we go +2 in all directions
        assert_eq!(*line.x.iter().max().unwrap(), 7);
        assert_eq!(*line.x.iter().min().unwrap(), 3);
        assert_eq!(*line.y.iter().max().unwrap(), 7);
        assert_eq!(*line.y.iter().min().unwrap(), 3);
        assert_eq!(line.dist, 0);

        let peg_a = Peg::new(5, 5);
        let peg_b = Peg::new(6, 6);
        let line = peg_a.line_to(&peg_b, 2, None);
        assert_eq!(*line.x.iter().min().unwrap(), 4);
        assert_eq!(*line.x.iter().max().unwrap(), 7);
        assert_eq!(*line.y.iter().min().unwrap(), 4);
        assert_eq!(*line.y.iter().max().unwrap(), 7);
        assert_eq!(line.dist, 1);
    }

    #[test]
    fn peg_line_to_width_min_max() {
        let peg_a = Peg::new(0, 5);
        let peg_b = Peg::new(10, 5);
        let line = peg_a.line_to(&peg_b, 2, Some((0, 10, 3, 7)));
        assert_eq!(*line.x.iter().max().unwrap(), 10);
        assert_eq!(*line.x.iter().min().unwrap(), 0);

        assert_eq!(*line.y.iter().max().unwrap(), 6);
        assert_eq!(*line.y.iter().min().unwrap(), 4);
    }

    #[test]
    fn peg_around() {
        let peg = Peg::new(10, 10);
        let (x_coords, y_coords) = peg.around(1);
        assert_eq!(x_coords, vec![9, 10, 10, 10, 11]);
        assert_eq!(y_coords, vec![10, 9, 10, 11, 10]);
    }

    #[test]
    fn peg_jitter() {
        let peg = Peg::new(10, 10);
        let jitter = 2;
        let peg_jitter = peg.with_jitter(jitter);
        assert!(peg_jitter.x <= (peg.x as i64 + jitter) as u32);
        assert!(peg_jitter.x >= (peg.x as i64 - jitter) as u32);
        assert_eq!(peg_jitter.id, peg.id);
    }
}
