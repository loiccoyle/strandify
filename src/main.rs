use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::iter::zip;
use std::path::PathBuf;

use clap::Parser;
use image::imageops;
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
    let output_file = args.output.as_ref().map(PathBuf::from);
    let output_file_extension = match &output_file {
        Some(output) => Some(
            output
                .extension()
                .ok_or("Could not determine OUTPUT extension.")?
                .to_owned(),
        ),
        None => None,
    };

    // Handle blueprint json as input
    if input_file
        .extension()
        .ok_or("Could not determine INPUT extension.")?
        == "json"
    {
        info!("Loading blueprint from file '{input_file:?}'");

        let bp = blueprint::Blueprint::from_file(input_file)?;

        let output_file = output_file
            .as_ref()
            .ok_or("Output file required to render output image.")?;

        return if output_file_extension.unwrap() == "json" {
            // This case is probably useless, read the json then write the json...
            info!("Writing blueprint to {output_file:?}.");
            bp.to_file(output_file)
        } else {
            info!("Rendering blueprint to {output_file:?}.");
            let yarn = peg::Yarn::new(args.yarn_width, args.yarn_opacity, (0, 0, 0));
            bp.render(output_file, &yarn, !args.verbose.is_silent())
        };
    }

    let img_rgb = utils::open_img_transparency_to_white(PathBuf::from(args.input));

    let img = if args.project_to_yarn_color
        && (args.yarn_color.r != args.yarn_color.g || args.yarn_color.g != args.yarn_color.b)
    {
        info!("Projecting to yarn color");
        // otherwise project along the color vector
        // convert to [0, 1]
        let yarn_color_float = (
            args.yarn_color.r as f32 / 255.,
            args.yarn_color.g as f32 / 255.,
            args.yarn_color.b as f32 / 255.,
        );
        // compute the norm of the yarn color vector
        let color_norm = ((yarn_color_float.0).powi(2)
            + (yarn_color_float.1).powi(2)
            + (yarn_color_float.2).powi(2))
        .sqrt();

        info!("projecting along {yarn_color_float:?}");
        let mut img = image::GrayImage::new(img_rgb.width(), img_rgb.height());
        info!("norm: {color_norm}");
        // project the color space onto the yarn color vector
        let mut value_min = 255;
        let mut value_max = 0;
        for (pixel, pixel_rgb) in zip(img.pixels_mut(), img_rgb.pixels()) {
            let intensity = (pixel_rgb.0[0] as f32 * yarn_color_float.0
                + pixel_rgb.0[1] as f32 * yarn_color_float.1
                + pixel_rgb.0[2] as f32 * yarn_color_float.2)
                / (yarn_color_float.0 + yarn_color_float.1 + yarn_color_float.2);

            let adjusted = intensity
                - 0.5
                    * ((pixel_rgb.0[0] as f32 + pixel_rgb.0[1] as f32 + pixel_rgb.0[2] as f32)
                        - intensity);
            let value = (255. - adjusted).clamp(0., 255.) as u8;
            pixel.0 = [value];
            if value < value_min {
                value_min = value;
            }
            if value > value_max {
                value_max = value;
            }
        }
        // min max scale to use the full color space
        for pixel in img.pixels_mut() {
            pixel.0[0] = (255. * (pixel.0[0] - value_min) as f32 / (value_max - value_min) as f32)
                .round() as u8;
        }
        img
    } else {
        info!("converting to grayscale");
        // if yarn is grey scale, just convert the img to black and white
        imageops::grayscale(&img_rgb)
    };

    let (width, height) = img_rgb.dimensions();
    let min_dim = min(width, height);
    let margin = (min_dim as f64 * args.peg_margin).round() as u32;
    info!("Peg margin: {margin}px");

    let mut pegs: Vec<peg::Peg>;
    // Handle the generation of pegs and computation of blueprint
    if let Some(peg_path) = args.load_pegs {
        // Load pegs from file
        info!("Reading {peg_path:?}");
        let reader = BufReader::new(File::open(peg_path)?);
        pegs = serde_json::from_reader(reader)?;
    } else {
        info!("Generating pegs");
        let center = (width / 2, height / 2);
        // Generate pegs from scratch
        let (peg_coords_x, peg_coords_y) = if args.peg_shape == "circle" {
            info!("Using circle peg distribution");
            utils::circle_coords((min_dim - 2 * margin) / 2, center, args.peg_number)
        } else if args.peg_shape == "square" {
            info!("Using square peg distribution");
            utils::square_coords(min_dim - 2 * margin, center, args.peg_number)
        } else if args.peg_shape == "border" {
            info!("Using border peg distribution");
            utils::rectangle_coords(
                width - 2 * margin,
                height - 2 * margin,
                center,
                args.peg_number,
            )
        } else {
            return Err(format!("Unrecognized SHAPE '{}'", args.peg_shape).into());
        };

        pegs = vec![];
        for (id, (peg_x, peg_y)) in zip(peg_coords_x, peg_coords_y).enumerate() {
            pegs.push(peg::Peg::new(peg_x, peg_y, id as u16));
        }

        // Add jitter to pegs
        if let Some(jitter) = args.peg_jitter {
            pegs = pegs
                .iter()
                .map(|peg| peg.with_jitter(jitter as i64))
                .collect();
        }
    }

    if let Some(peg_path) = args.save_pegs {
        info!("Saving pegs to {peg_path:?}");
        serde_json::to_writer(File::create(peg_path)?, &pegs)?
    }

    if let Some(output_file) = output_file {
        let skip_peg_within = args.peg_skip_within.unwrap_or(min_dim / 8);
        info!("Skip peg within: {skip_peg_within:?}px");

        let config = pather::PatherConfig::new(
            args.iterations,
            args.lighten_factor,
            5,
            skip_peg_within,
            !args.verbose.is_silent(),
        );
        debug!("config: {config:?}");

        let mut string_pather = pather::Pather::new(
            img.clone(),
            pegs.clone(),
            peg::Yarn::new(
                args.yarn_width,
                args.yarn_opacity,
                (args.yarn_color.r, args.yarn_color.g, args.yarn_color.b),
            ),
            config.clone(),
        );
        string_pather.populate_line_cache()?;

        let mut bp = string_pather.compute()?;
        if args.transparent {
            bp.background = None;
        }
        bp.render_scale = args.output_scale;

        if output_file_extension.unwrap() == "json" {
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
