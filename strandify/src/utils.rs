use std::{error::Error, f64::consts::PI, path::Path};

use crate::peg::Peg;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;

/// Compute the coords of evenly spaced points around a circle
///
/// # Arguments
///
/// * `radius`: the radius of the circle
/// * (`center_x`,` center_y`): the coords of the center point
/// * `n_points`: the number of points
pub fn circle_coords(
    radius: u32,
    (center_x, center_y): (u32, u32),
    n_points: usize,
) -> (Vec<u32>, Vec<u32>) {
    let radius = radius as f64;
    let angle = 2. * PI / n_points as f64;
    let mut points_x = vec![];
    let mut points_y = vec![];
    let mut i_angle: f64;
    for i in 0..n_points {
        i_angle = i as f64 * angle;
        points_x.push((radius * (i_angle).cos() + center_x as f64).round() as u32);
        points_y.push((radius * (i_angle).sin() + center_y as f64).round() as u32);
    }
    (points_x, points_y)
}

/// Get the coords of evenly spaced points along a line.
///
/// # Arguments
///
/// * `start`: the start point
/// * `end`: the end point
/// * `n_points`: the number of points
pub fn line_coords(start: (u32, u32), end: (u32, u32), num_points: usize) -> (Vec<u32>, Vec<u32>) {
    let (x1, y1): (f64, f64) = (start.0 as f64, start.1 as f64);
    let (x2, y2): (f64, f64) = (end.0 as f64, end.1 as f64);
    let dx = (x2 - x1) / num_points as f64;
    let dy = (y2 - y1) / num_points as f64;

    (0..num_points)
        .map(|i| {
            let t = i as f64;
            ((x1 + t * dx).round() as u32, (y1 + t * dy).round() as u32)
        })
        .unzip()
}

/// Compute the coords of evenly spaced points around a square.
///
/// # Arguments
///
/// * `length`: the legnth of the side of the square
/// * (`center_x`,` center_y`): the coords of the center point
/// * `n_points`: the number of points
pub fn square_coords(top_left: (u32, u32), length: u32, n_points: usize) -> (Vec<u32>, Vec<u32>) {
    let (x0, y0) = top_left;
    let top_right = (x0 + length, y0);
    let bottom_right = (x0 + length, y0 + length);
    let bottom_left = (x0, y0 + length);

    let n_points_side = n_points / 4;
    let top = line_coords(top_left, top_right, n_points_side);
    let right = line_coords(top_right, bottom_right, n_points_side);
    let bottom = line_coords(bottom_right, bottom_left, n_points_side);
    let left = line_coords(bottom_left, top_left, n_points_side);

    (
        [top.0, right.0, bottom.0, left.0].concat(),
        [top.1, right.1, bottom.1, left.1].concat(),
    )
}

/// Compute the coords of evenly spaced points around a rectangle.
///
/// # Arguments
///
/// * `width`: the width of the side of the rectangle.
/// * `height`: the height of the side of the rectangle.
/// * (`center_x`,` center_y`): the coords of the center point
/// * `n_points`: the number of points
pub fn rectangle_coords(
    top_left: (u32, u32),
    width: u32,
    height: u32,
    n_points: usize,
) -> (Vec<u32>, Vec<u32>) {
    let (x0, y0) = top_left;
    let top_right = (x0 + width, y0);
    let bottom_right = (x0 + width, y0 + height);
    let bottom_left = (x0, y0 + height);

    let perimeter = 2 * width + 2 * height;
    let width_points = (n_points as u32 * width / perimeter) as usize;
    let height_points = (n_points as u32 * height / perimeter) as usize;
    debug!("Points along (width, height): ({width_points}, {height_points})");

    let top = line_coords(top_left, top_right, width_points);
    let right = line_coords(top_right, bottom_right, height_points);
    let bottom = line_coords(bottom_right, bottom_left, width_points);
    let left = line_coords(bottom_left, top_left, height_points);

    (
        [top.0, right.0, bottom.0, left.0].concat(),
        [top.1, right.1, bottom.1, left.1].concat(),
    )
}

/// Get the pixels around a point within `radius`.
///
/// # Arguments
///
/// *`(center_x, center_y)`: Center around which to fetch surrounding pixels.
/// *`radius`: Pixel radius around peg.
pub fn pixels_around((center_x, center_y): (u32, u32), radius: u32) -> (Vec<u32>, Vec<u32>) {
    let mut x_coords: Vec<u32> = vec![];
    let mut y_coords: Vec<u32> = vec![];
    let radius = radius as i64;
    let center_x = center_x as i64;
    let center_y = center_y as i64;
    for x in -radius..radius + 1 {
        for y in -radius..radius + 1 {
            if x * x + y * y <= radius * radius {
                x_coords.push((center_x + x) as u32);
                y_coords.push((center_y + y) as u32);
            }
        }
    }
    (x_coords, y_coords)
}

pub(crate) fn progress_style() -> Result<ProgressStyle, Box<dyn Error>> {
    Ok(ProgressStyle::with_template(
        "{msg}: {wide_bar} {elapsed_precise} {pos}/{len}",
    )?)
}

pub(crate) fn pbar(len: u64, hidden: bool) -> Result<ProgressBar, Box<dyn Error>> {
    let style = progress_style()?;
    Ok(if hidden {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(len)
    }
    .with_style(style))
}

pub(crate) fn spinner(hidden: bool) -> ProgressBar {
    if hidden {
        ProgressBar::hidden()
    } else {
        ProgressBar::new_spinner()
    }
}

pub(crate) fn abs_diff<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd + std::ops::Sub<Output = T>,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// Create a hash key from two pegs. Used to construct the [`Pather::line_cache`](crate::pather::Pather::line_cache).
pub fn hash_key(peg_a: &Peg, peg_b: &Peg) -> (usize, usize) {
    if peg_a.id < peg_b.id {
        (peg_a.id, peg_b.id)
    } else {
        (peg_b.id, peg_a.id)
    }
}

/// Open an image and set all fully transparent pixels to white.
pub fn open_img_transparency_to_white<P: AsRef<Path>>(
    image_file: P,
) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
    let mut img_rgba = image::open(image_file)?.into_rgba8();
    for pixel in img_rgba.pixels_mut() {
        // replace fully transparent pixel with white
        if pixel.0[3] == 0 {
            pixel.0 = [255, 255, 255, 255]
        }
    }
    Ok(image::DynamicImage::ImageRgba8(img_rgba).to_rgb8())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(1., 2.), 1.);
        assert_eq!(abs_diff(2., 1.), 1.);
        assert_eq!(abs_diff(1, 2), 1);
        assert_eq!(abs_diff(2, 1), 1);
    }

    #[test]
    fn test_line_coords() {
        // 5 points between 0 and 10
        // 0 1 2 3 4 5 6 7 8 9 10
        // *   *   *   *   *
        let n_points = 5;
        let (x1, y1) = (0, 0);
        let (x2, y2) = (10, 0);
        let (x, y) = line_coords((x1, y1), (x2, y2), n_points);
        assert_eq!(x.len(), n_points);
        assert_eq!(y.len(), n_points);
        assert_eq!(x.first(), Some(&x1));
        assert_eq!(y.first(), Some(&y1));

        let (x_end, y_end) = (8, 0);
        assert_eq!(x.last(), Some(&x_end));
        assert_eq!(y.last(), Some(&y_end));
    }
}
