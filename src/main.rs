use clap::Parser;
use env_logger;
use log::debug;
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
    let knitter = knitter::Knitter::from_file(
        PathBuf::from(args.image),
        vec![knitter::Peg::new(0, 0, 1)],
        knitter::Yarn::new(1, 1.0),
        128,
    );
    debug!("knitter: {:?}", knitter);
}
