use rand::{thread_rng, Rng};
use std::f64::consts::PI;

use indicatif::ProgressStyle;

/// Compute the coords of evenly spaced points around a circle
///
/// # Arguments
///
/// * `radius` - the radius of the circle
/// * (`center_x`,` center_y`) - the coords of the center point
/// * `n_division` - the number of divisions
/// * `jitter` - add some optional random angular jitter to the points in rad
pub fn circle_coords(
    radius: f64,
    (center_x, center_y): (u32, u32),
    n_division: u32,
    jitter: Option<f64>,
) -> (Vec<u32>, Vec<u32>) {
    let mut rng = thread_rng();
    let angle = 2. * PI / n_division as f64;
    let angle_jitter = match jitter {
        Some(angle_jitter) => angle_jitter,
        None => 0.,
    };
    let mut points_x = vec![];
    let mut points_y = vec![];
    let mut i_angle: f64;
    for i in 0..n_division {
        i_angle = i as f64 * angle;
        if jitter.is_some() {
            i_angle += rng.gen_range(-angle_jitter..angle_jitter);
        }
        points_x.push((radius * (i_angle).cos() + center_x as f64).round() as u32);
        points_y.push((radius * (i_angle).sin() + center_y as f64).round() as u32);
    }
    (points_x, points_y)
}

pub fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template("{msg}: {wide_bar} {elapsed_precise} {pos}/{len}").unwrap()
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

// pub fn abs_diff(a: &u32, b: &u32) -> u32 {
//     if a > b {
//         a - b
//     } else {
//         b - a
//     }
// }
