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
    let output_file = PathBuf::from(args.output);

    let (width, height) = img.dimensions();
    let min_dim = min(width, height) as f64;
    let dist = (min_dim * (1. - args.peg_margin)).round() as u32;
    let center = (width / 2, height / 2);
    let skip_peg_within = args.peg_skip_within.unwrap_or(dist / 8);
    info!("Skip peg within: {skip_peg_within:?}");

    let (mut peg_coords_x, mut peg_coords_y) = if args.peg_shape == "circle" {
        info!("Using circle distribution");
        utils::circle_coords(dist / 2, center, args.peg_number)
    } else if args.peg_shape == "square" {
        info!("Using square distribution");
        utils::square_coords(dist, center, args.peg_number)
    } else {
        return Err("Unrecognized PEG_SHAPE".to_string());
    };

    if args.peg_jitter.is_some() {
        info!("Adding jitter");
        (peg_coords_x, peg_coords_y) = utils::add_jitter(
            (peg_coords_x, peg_coords_y),
            args.peg_jitter.unwrap() as i64,
        )
    }

    let mut pegs: Vec<peg::Peg> = vec![];
    for (id, (peg_x, peg_y)) in zip(peg_coords_x, peg_coords_y).enumerate() {
        pegs.push(peg::Peg::new(peg_x, peg_y, id as u16));
    }

    let config = pather::PatherConfig::new(
        args.iterations,
        args.lighten_factor,
        5,
        skip_peg_within,
        !args.verbose.is_silent(),
    );
    let yarn = peg::Yarn::new(args.yarn_width, args.yarn_opacity);
    info!("config: {config:?}");
    info!("yarn: {yarn:?}");

    let string_art = pather::Pather::new(img, pegs, yarn, config);
    let blueprint = string_art.peg_order();

    if output_file.extension().unwrap() == "svg" {
        let svg_img = string_art.render_svg(&blueprint);
        svg::save(output_file, &svg_img).unwrap();
    } else {
        let img = string_art.render(&blueprint);
        img.save(output_file).unwrap();
    }

    Ok(())
}
