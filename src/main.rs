use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::iter::zip;
use std::path::PathBuf;

use clap::Parser;
use log::{debug, info};

use stringart::blueprint;
use stringart::cli;
use stringart::pather;
use stringart::peg;
use stringart::utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Arguments::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    debug!("cli args: {:?}", args);
    let input_file = PathBuf::from(&args.input);
    // Handle blueprint json as input
    if input_file.extension().unwrap() == "json" {
        info!("Loading blueprint from file '{input_file:?}'");
        let output_file = PathBuf::from(args.output.as_ref().ok_or("No OUTPUT provided.")?);

        let bp = blueprint::Blueprint::from_file(input_file)?;

        return if output_file.extension().unwrap() == "json" {
            // This case is probably useless, read the json then write the json...
            info!("Writing blueprint to {output_file:?}.");
            bp.to_file(&output_file).map_err(|err| err.into())
        } else {
            info!("Rendering blueprint to {output_file:?}.");
            let yarn = peg::Yarn::new(args.yarn_width, args.yarn_opacity);
            bp.render(&output_file, &yarn, !args.verbose.is_silent())
        };
    }
    // Handle the generation of pegs and computation of blueprint
    let img = utils::open_img_transparency_to_white(PathBuf::from(args.input));

    let (width, height) = img.dimensions();
    let dist = ((min(width, height) as f64) * (1. - args.peg_margin)).round() as u32;

    let mut pegs: Vec<peg::Peg>;
    if let Some(peg_path) = args.load_pegs {
        // Load pegs from file
        info!("Reading {peg_path:?}");
        let reader = BufReader::new(File::open(peg_path)?);
        pegs = serde_json::from_reader(reader)?;
    } else {
        info!("Generating pegs");
        let center = (width / 2, height / 2);
        // Generate pegs from scratch
        let (mut peg_coords_x, mut peg_coords_y) = if args.peg_shape == "circle" {
            info!("Using circle peg distribution");
            utils::circle_coords(dist / 2, center, args.peg_number)
        } else if args.peg_shape == "square" {
            info!("Using square peg distribution");
            utils::square_coords(dist, center, args.peg_number)
        } else {
            return Err(format!("Unrecognized SHAPE '{:?}'", args.peg_shape).into());
        };

        if let Some(jitter) = args.peg_jitter {
            info!("Adding jitter to pegs");
            (peg_coords_x, peg_coords_y) =
                utils::add_jitter((peg_coords_x, peg_coords_y), jitter as i64)
        }

        pegs = vec![];
        for (id, (peg_x, peg_y)) in zip(peg_coords_x, peg_coords_y).enumerate() {
            pegs.push(peg::Peg::new(peg_x, peg_y, id as u16));
        }
    }

    if let Some(peg_path) = args.save_pegs {
        info!("Saving pegs to {peg_path:?}");
        serde_json::to_writer(File::create(peg_path)?, &pegs)?
    }

    if let Some(output_file) = args.output {
        let output_file = PathBuf::from(output_file);
        let skip_peg_within = args.peg_skip_within.unwrap_or(dist / 8);
        info!("Skip peg within: {skip_peg_within:?}px");

        let config = pather::PatherConfig::new(
            args.iterations,
            args.lighten_factor,
            5,
            skip_peg_within,
            !args.verbose.is_silent(),
        );
        let yarn = peg::Yarn::new(args.yarn_width, args.yarn_opacity);
        debug!("config: {config:?}");
        debug!("yarn: {yarn:?}");

        let string_pather = pather::Pather::new(img, pegs, yarn, config);
        let bp = string_pather.compute();

        if output_file.extension().unwrap() == "json" {
            info!("Writing blueprint to {output_file:?}.");
            bp.to_file(&output_file)?;
        } else {
            info!("Rendering blueprint to {output_file:?}.");
            bp.render(
                &output_file,
                &string_pather.yarn,
                string_pather.config.progress_bar,
            )?;
        }
    }

    Ok(())
}
