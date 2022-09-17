use clap::Parser;
use clap_verbosity_flag::Verbosity;
use image::io::Reader;
use std::path::PathBuf;

fn check_file_exists(image: &str) -> Result<(), String> {
    let image_file = PathBuf::from(image);
    if image_file.exists() {
        let reader = Reader::open(&image_file)
            .unwrap()
            .with_guessed_format()
            .unwrap();
        if reader.format().is_some() {
            Ok(())
        } else {
            Err(format!("File {:?} format not supported.", image_file))
        }
    } else {
        Err(format!("File {:?} does not exist.", image_file))
    }
}

fn number_between_0_and_1(value: &str) -> Result<f64, String> {
    let value: f64 = value
        .parse()
        .map_err(|_| format!("{:?} is not a number", value))?;

    if 0. <= value && value <= 1. {
        Ok(value)
    } else {
        Err(format!("Value '{:?}' should be between 0 and 1", value))
    }
}

#[derive(Parser, Debug)]
#[clap(author = "Loic Coyle")]
/// KnitArt
pub struct Arguments {
    /// Input image file
    #[clap(validator=check_file_exists)]
    pub image: String,
    /// Output image file
    #[clap(default_value = "knitart.png")]
    pub output: String,
    /// Number of iterations
    #[clap(short, long, value_parser, default_value_t = 20000)]
    pub iterations: u32,
    /// Number of pegs
    #[clap(short = 'n', long, value_parser, default_value_t = 200)]
    pub peg_number: u32,
    /// Margin between pegs and image edge [0, 1]
    #[clap(short = 'm', long, value_parser=number_between_0_and_1, default_value_t = 0.05)]
    pub peg_margin: f64,
    /// Add angular jitter to the pegs, in rad [default: 2*pi/PEG_NUMBER*5]
    #[clap(short = 'j', value_parser, long)]
    pub peg_jitter: Option<f64>,
    /// Don't connect pegs within pixel distance [default: radius/4]
    #[clap(short = 's', value_parser, long)]
    pub peg_skip_within: Option<u32>,
    /// Yarn opacity [0, 1]
    #[clap(short, long, value_parser=number_between_0_and_1, default_value_t = 0.02)]
    pub opacity: f64,
    /// Encourages peg exploration at the expense of contrast, should be greater than 1
    #[clap(short, long, value_parser, default_value_t = 1.05)]
    pub lighten_factor: f64,
    /// Verbosity level.
    #[clap(flatten)]
    pub verbose: Verbosity,
}
