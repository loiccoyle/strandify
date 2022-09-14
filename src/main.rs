use clap::Parser;
use env_logger;
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

    let width = img.width();
    let height = img.height();
    let radius = (min(width, height) as f64 * args.peg_radius_scale / 2.).round();
    let center = (width / 2, height / 2);
    let jitter = match args.peg_jitter {
        Some(_) => args.peg_jitter,
        None => Some(2. * PI / (args.pegs as f64 * 5.)),
    };
    info!("Peg jitter: {jitter:?}");

    let (peg_coords_x, peg_coords_y) = utils::circle_coords(radius, center, args.pegs, jitter);

    let mut pegs: Vec<peg::Peg> = vec![];
    for (id, (peg_x, peg_y)) in zip(peg_coords_x, peg_coords_y).enumerate() {
        pegs.push(peg::Peg::new(peg_x, peg_y, id as u16));
    }

    let config = knitter::KnitterConfig::new(
        args.iterations,
        args.lighten_factor,
        match args.peg_exclude_neighbours {
            Some(n_neighbours) => n_neighbours,
            None => (pegs.len() / 20) as u16,
        },
        5,
    );
    let yarn = peg::Yarn::new(1, args.opacity);
    info!("config: {config:?}");
    info!("yarn: {yarn:?}");

    let knitart = knitter::Knitter::new(img, pegs, yarn, config);
    let blueprint = knitart.peg_order();
    let knit_img = knitart.knit(&blueprint);
    knit_img.save(args.output).unwrap();
}
