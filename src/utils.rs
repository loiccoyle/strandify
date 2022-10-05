use rand::{thread_rng, Rng};
use std::{f64::consts::PI, path::Path};

use crate::peg::Peg;

use image::imageops;
use indicatif::{ProgressBar, ProgressStyle};

/// Compute the coords of evenly spaced points around a circle
///
/// # Arguments
///
/// * `radius` - the radius of the circle
/// * (`center_x`,` center_y`) - the coords of the center point
/// * `n_division` - the number of divisions
pub fn circle_coords(
    radius: u32,
    (center_x, center_y): (u32, u32),
    n_division: u32,
) -> (Vec<u32>, Vec<u32>) {
    let radius = radius as f64;
    let angle = 2. * PI / n_division as f64;
    let mut points_x = vec![];
    let mut points_y = vec![];
    let mut i_angle: f64;
    for i in 0..n_division {
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
/// * `length` - the legnth of the side of the square
/// * (`center_x`,` center_y`) - the coords of the center point
/// * `n_pegs` - the number of pegs
pub fn square_coords(
    length: u32,
    (center_x, center_y): (u32, u32),
    n_pegs: u32,
) -> (Vec<u32>, Vec<u32>) {
    let dist_between: f64 = 4. * length as f64 / n_pegs as f64;

    let top_left_x = center_x - length / 2;
    let top_left_y = center_y - length / 2;

    let mut x_coords: Vec<u32> = vec![];
    let mut y_coords: Vec<u32> = vec![];

    for i in 0..n_pegs {
        let dist = (i as f64 * dist_between) as u32;
        let side = dist / length;
        let rem = dist % length;
        if side == 0 {
            // top
            x_coords.push(top_left_x + rem);
            y_coords.push(top_left_y);
        } else if side == 1 {
            // right
            x_coords.push(top_left_x + length);
            y_coords.push(top_left_y + rem);
        } else if side == 2 {
            // bottom
            x_coords.push(top_left_x + length - rem);
            y_coords.push(top_left_y + length);
        } else if side == 3 {
            // left
            x_coords.push(top_left_x);
            y_coords.push(top_left_y + length - rem);
        }
    }
    (x_coords, y_coords)
}

/// Add 2d jitter to pegs.
///
/// # Arguments
///
/// * `(x_coords, y_coords)` - Vectors of x,y coords.
/// * `jitter` - Amount of jitter to add, in pixels.
pub fn add_jitter((x_coords, y_coords): (Vec<u32>, Vec<u32>), jitter: i64) -> (Vec<u32>, Vec<u32>) {
    let mut rng = thread_rng();

    let x_coords_jit: Vec<u32> = x_coords
        .into_iter()
        .map(|x| (x as i64 + rng.gen_range(-jitter..jitter)) as u32)
        .collect();
    let y_coords_jit: Vec<u32> = y_coords
        .into_iter()
        .map(|x| (x as i64 + rng.gen_range(-jitter..jitter)) as u32)
        .collect();

    (x_coords_jit, y_coords_jit)
}

/// Get the pixels around a peg within` radius`
///
/// # Arguments
///
/// *`(center_x, center_y)` - center around which to fetch surrounding pixels
/// *`radius` - pixel radius around peg
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

pub fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template("{msg}: {wide_bar} {elapsed_precise} {pos}/{len}").unwrap()
}

pub fn pbar(len: u64, hidden: bool) -> ProgressBar {
    if hidden {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(len)
    }
    .with_style(progress_style())
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

/// Open an image and set all fully transparent pixels to white
pub fn open_img_transparency_to_white<P: AsRef<Path>>(
    image_file: P,
) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let mut img_rgba = image::open(image_file).unwrap().into_rgba8();
    for pixel in img_rgba.pixels_mut() {
        // replace fully transparent pixel with white
        if pixel.0[3] == 0 {
            pixel.0 = [255, 255, 255, 255]
        }
    }
    imageops::grayscale(&img_rgba)
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
