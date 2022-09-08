use clap::Parser;
use env_logger;
use log::debug;
use std::cmp::min;
use std::iter::zip;
use std::path::PathBuf;

mod cli;
mod knitter;
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
    let radius = (min(width, height) as f64 * 0.9 / 2.).round();
    let center = (width / 2, height / 2);

    let (peg_coords_x, peg_coords_y) = utils::circle_coords(radius, center, 128);
    let mut peg_vec: Vec<knitter::Peg> = vec![];
    for (id, (peg_x, peg_y)) in zip(peg_coords_x, peg_coords_y).enumerate() {
        peg_vec.push(knitter::Peg::new(peg_x, peg_y, id as u16))
    }

    let mut knitart = knitter::Knitter::new(img, peg_vec, knitter::Yarn::new(1, 1.0), 5096);
    knitart.populate_line_cache();
    let order = knitart.peg_order();
    let knit_img = knitart.knit(&order);
    knit_img.save("knitart.png").unwrap();
}
