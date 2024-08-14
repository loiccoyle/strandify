use image;
use std::iter::zip;

#[derive(Debug)]
pub struct Line {
    /// X coordinates of the pixels in the line.
    pub x: Vec<u32>,
    /// Y coordinates of the pixels in the line.
    pub y: Vec<u32>,
    /// The distance of the line in pixels.
    pub dist: u32,
}

/// Helper struct that represents a line between 2 [`Pegs`](crate::peg::Peg). Holds the vectors of the pixel coordinates of the line.
impl Line {
    /// Creates a new [`Line`].
    pub fn new(x: Vec<u32>, y: Vec<u32>, dist: u32) -> Self {
        assert_eq!(x.len(), y.len(), "`x` and `y` should have the same length");
        Self { x, y, dist }
    }

    /// Returns the length of this [`Line`].
    pub fn len(&self) -> usize {
        self.x.len()
    }

    /// Returns the is empty of this [`Line`].
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    /// Returns the zip of this [`Line`].
    ///
    /// Zips over both coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use strandify::line::Line;
    /// let line = Line::new(vec![0, 1], vec![0, 0], 1);
    /// for (x, y) in line.zip() {
    ///     println!("x: {x:?}, y: {y:?}");
    /// }
    /// assert_eq!(line.zip().len(), 2);
    /// ```
    pub fn zip(&self) -> std::iter::Zip<std::slice::Iter<u32>, std::slice::Iter<u32>> {
        zip(&self.x, &self.y)
    }

    /// Returns the copy of this [`Line`].
    pub fn copy(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone(), self.dist)
    }

    /// Compute the loss of this [`Line`] over the provided single channel [`image::ImageBuffer`].
    ///
    /// # Args:
    /// - `image`: The [`image::ImageBuffer`] to compute the loss over.
    ///
    /// # Returns:
    /// - `f64`: The loss of this [`Line`].
    pub fn loss(&self, image: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>) -> f64 {
        self.zip()
            .map(|(x, y)| image.get_pixel(*x, *y))
            .fold(0.0, |acc, &pixel| acc + (pixel.0[0] as f64))
            / (255. * self.len() as f64)
    }
}
