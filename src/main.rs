use clap::Parser;
use log::{debug, info};
use std::cmp::min;
use std::f64::consts::PI;
use std::iter::zip;
use std::path::PathBuf;

mod cli;
mod knitter;
mod peg;
mod utils;

fn main() {
    let args = cli::Arguments::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    debug!("cli args: {:?}", args);
    let img = image::open(PathBuf::from(args.image)).unwrap().into_luma8();

    let (width, height) = img.dimensions();
    let min_dim = min(width, height) as f64;
    let dist = (min_dim * (1. - args.peg_margin)).round() as u32;
    let center = (width / 2, height / 2);
    let jitter = args
        .peg_jitter
        .unwrap_or(2. * PI / (args.peg_number as f64 * 5.));
    let skip_peg_within = args.peg_skip_within.unwrap_or(dist / 4);
    info!("Peg jitter: {jitter:?}");
    info!("Skip peg within: {skip_peg_within:?}");

    let (peg_coords_x, peg_coords_y) =
        utils::circle_coords(dist / 2, center, args.peg_number, Some(jitter));
    // utils::square_coords(dist, center, args.peg_number, Some(5));

    let mut pegs: Vec<peg::Peg> = vec![];
    for (id, (peg_x, peg_y)) in zip(peg_coords_x, peg_coords_y).enumerate() {
        pegs.push(peg::Peg::new(peg_x, peg_y, id as u16));
    }

    let config =
        knitter::KnitterConfig::new(args.iterations, args.lighten_factor, 5, skip_peg_within);
    let yarn = peg::Yarn::new(1, args.opacity);
    info!("config: {config:?}");
    info!("yarn: {yarn:?}");

    let knitart = knitter::Knitter::new(img, pegs, yarn, config);
    let blueprint = knitart.peg_order();
    let knit_img = knitart.knit(&blueprint);
    knit_img.save(args.output).unwrap();
}
