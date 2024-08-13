use std::{error::Error, f64::consts::PI, path::Path};

use crate::peg::Peg;
use image::ImageBuffer;
use indicatif::{ProgressBar, ProgressStyle};

/// Compute the coords of evenly spaced points around a circle
///
/// # Arguments
///
/// * `radius`: the radius of the circle
/// * (`center_x`,` center_y`): the coords of the center point
/// * `n_pegs`: the number of pegs
pub fn circle_coords(
    radius: u32,
    (center_x, center_y): (u32, u32),
    n_pegs: u32,
) -> (Vec<u32>, Vec<u32>) {
    let radius = radius as f64;
    let angle = 2. * PI / n_pegs as f64;
    let mut points_x = vec![];
    let mut points_y = vec![];
    let mut i_angle: f64;
    for i in 0..n_pegs {
        i_angle = i as f64 * angle;
        points_x.push((radius * (i_angle).cos() + center_x as f64).round() as u32);
        points_y.push((radius * (i_angle).sin() + center_y as f64).round() as u32);
    }
    (points_x, points_y)
}

/// Compute the coords of evenly spaced points around a square
///
/// # Arguments
///
/// * `length`: the legnth of the side of the square
/// * (`center_x`,` center_y`): the coords of the center point
/// * `n_pegs`: the number of pegs
pub fn square_coords(
    length: u32,
    (center_x, center_y): (u32, u32),
    n_pegs: u32,
) -> (Vec<u32>, Vec<u32>) {
    rectangle_coords(length, length, (center_x, center_y), n_pegs)
}

/// Compute the coords of evenly spaced points around a rectangle
///
/// # Arguments
///
/// * `width`: the width of the side of the rectangle.
/// * `height`: the height of the side of the rectangle.
/// * (`center_x`,` center_y`): the coords of the center point
/// * `n_pegs`: the number of pegs
pub fn rectangle_coords(
    width: u32,
    height: u32,
    (center_x, center_y): (u32, u32),
    n_pegs: u32,
) -> (Vec<u32>, Vec<u32>) {
    let (center_x, center_y) = (center_x as f64, center_y as f64);
    let width = width as f64;
    let height = height as f64;
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    let perimeter = 2.0 * (width + height);
    let step_length = perimeter / n_pegs as f64;

    let mut x_coords: Vec<u32> = Vec::with_capacity(n_pegs as usize);
    let mut y_coords: Vec<u32> = Vec::with_capacity(n_pegs as usize);

    // Current position in perimeter
    let mut current_distance = 0.0;

    for _ in 0..n_pegs {
        let point = if current_distance < width {
            // Bottom edge
            (
                center_x - half_width + current_distance,
                center_y - half_height,
            )
        } else if current_distance < width + height {
            // Right edge
            let dist = current_distance - width;
            (center_x + half_width, center_y - half_height + dist)
        } else if current_distance < 2.0 * width + height {
            // Top edge
            let dist = current_distance - width - height;
            (center_x + half_width - dist, center_y + half_height)
        } else {
            // Left edge
            let dist = current_distance - 2.0 * width - height;
            (center_x - half_width, center_y + half_height - dist)
        };

        x_coords.push(point.0.round() as u32);
        y_coords.push(point.1.round() as u32);

        current_distance += step_length;
    }

    (x_coords, y_coords)
}

/// Get the pixels around a peg within `radius`.
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

pub fn progress_style() -> Result<ProgressStyle, Box<dyn Error>> {
    Ok(ProgressStyle::with_template(
        "{msg}: {wide_bar} {elapsed_precise} {pos}/{len}",
    )?)
}

pub fn pbar(len: u64, hidden: bool) -> Result<ProgressBar, Box<dyn Error>> {
    let style = progress_style()?;
    Ok(if hidden {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(len)
    }
    .with_style(style))
}

pub fn spinner(hidden: bool) -> ProgressBar {
    if hidden {
        ProgressBar::hidden()
    } else {
        ProgressBar::new_spinner()
    }
}

pub fn abs_diff<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd + std::ops::Sub<Output = T>,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn hash_key(peg_a: &Peg, peg_b: &Peg) -> (u16, u16) {
    if peg_a.id < peg_b.id {
        (peg_a.id, peg_b.id)
    } else {
        (peg_b.id, peg_a.id)
    }
}

/// Open an image and set all fully transparent pixels to white.
pub fn open_img_transparency_to_white<P: AsRef<Path>>(
    image_file: P,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut img_rgba = image::open(image_file).unwrap().into_rgba8();
    for pixel in img_rgba.pixels_mut() {
        // replace fully transparent pixel with white
        if pixel.0[3] == 0 {
            pixel.0 = [255, 255, 255, 255]
        }
    }
    image::DynamicImage::ImageRgba8(img_rgba).to_rgb8()
}

type RgbImage = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;
pub fn split_channels(
    img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> (RgbImage, RgbImage, RgbImage) {
    let (width, height) = img.dimensions();

    // Create buffers for each channel
    let mut red: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut green: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut blue: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Iterate over the pixels and split the channels
    for (x, y, pixel) in img.enumerate_pixels() {
        let image::Rgb([r, g, b]) = *pixel;
        red.put_pixel(x, y, image::Rgb([r, 0, 0]));
        green.put_pixel(x, y, image::Rgb([0, g, 0]));
        blue.put_pixel(x, y, image::Rgb([0, 0, b]));
    }

    (red, green, blue)
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
}
