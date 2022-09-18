use clap::Parser;
use log::{debug, info};
use std::cmp::min;
use std::iter::zip;
use std::path::PathBuf;

mod cli;
mod pather;
mod peg;
mod utils;

fn main() -> Result<(), String> {
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
    let skip_peg_within = args.peg_skip_within.unwrap_or(dist / 8);
    info!("Skip peg within: {skip_peg_within:?}");

    let (peg_coords_x, peg_coords_y) = if args.peg_shape == "circle" {
        info!("Using circle distribution");
        utils::circle_coords(dist / 2, center, args.peg_number, args.peg_jitter)
    } else if args.peg_shape == "square" {
        info!("Using square distribution");
        utils::square_coords(dist, center, args.peg_number, args.peg_jitter)
    } else {
        return Err("Unrecognized PEG_SHAPE".to_string());
    };

    let mut pegs: Vec<peg::Peg> = vec![];
    for (id, (peg_x, peg_y)) in zip(peg_coords_x, peg_coords_y).enumerate() {
        pegs.push(peg::Peg::new(peg_x, peg_y, id as u16));
    }

    let config =
        pather::PatherConfig::new(args.iterations, args.lighten_factor, 5, skip_peg_within);
    let yarn = peg::Yarn::new(1, args.opacity);
    info!("config: {config:?}");
    info!("yarn: {yarn:?}");

    let knitart = pather::Pather::new(img, pegs, yarn, config);
    let blueprint = knitart.peg_order();
    let knit_img = knitart.knit(&blueprint);
    knit_img.save(args.output).unwrap();

    Ok(())
}
