use std::f64::consts::PI;

pub fn circle_coords(
    radius: f64,
    (center_x, center_y): (u32, u32),
    n_division: u64,
) -> (Vec<u32>, Vec<u32>) {
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
