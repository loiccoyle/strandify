use clap::Parser;
use clap_verbosity_flag::Verbosity;
use image::io::Reader;
use std::path::PathBuf;

fn check_file_exists(input: &str) -> Result<(), String> {
    let input_file = PathBuf::from(input);
    if input_file.exists() {
        let reader = Reader::open(&input_file)
            .unwrap()
            .with_guessed_format()
            .unwrap();
        if input_file.extension().unwrap() == "json" || reader.format().is_some() {
            Ok(())
        } else {
            Err(format!("File {:?} format not supported.", input_file))
        }
    } else {
        Err(format!("File {:?} does not exist.", input_file))
    }
}

fn number_between_0_and_1(value: &str) -> Result<f64, String> {
    let value: f64 = value
        .parse()
        .map_err(|_| format!("{:?} is not a number", value))?;

    if (0. ..=1.).contains(&value) {
        Ok(value)
    } else {
        Err(format!("Value '{:?}' should be between 0 and 1", value))
    }
}

#[derive(Parser, Debug)]
#[clap(author = "Loic Coyle")]
/// CLI utility to generate string art.
pub struct Arguments {
    /// Input image or blueprint json file
    #[clap(validator=check_file_exists)]
    pub input: String,
    /// Output file, either image format or json
    pub output: Option<String>,
    /// Number of iterations
    #[clap(short, long, value_parser, default_value_t = 4000)]
    pub iterations: u32,
    /// Peg distribution shape
    #[clap(short = 'S', long, value_parser=["circle", "square"], default_value = "circle", name="SHAPE")]
    pub peg_shape: String,
    /// Number of pegs
    #[clap(short = 'n', long, value_parser, default_value_t = 288)]
    pub peg_number: u32,
    /// Margin between pegs and image edge [0, 1]
    #[clap(short = 'm', long, value_parser=number_between_0_and_1, default_value_t = 0.05)]
    pub peg_margin: f64,
    /// Add jitter to the peg position
    #[clap(short = 'j', value_parser, long)]
    pub peg_jitter: Option<u32>,
    /// Don't connect pegs within pixel distance
    #[clap(short = 's', value_parser, long)]
    pub peg_skip_within: Option<u32>,
    /// Yarn opacity [0, 1]
    #[clap(short = 'o', long, value_parser=number_between_0_and_1, default_value_t = 0.3)]
    pub yarn_opacity: f64,
    /// Yarn width
    #[clap(short = 'w', long, value_parser, default_value_t = 1)]
    pub yarn_width: u32,
    /// How much to lighten the pixels at each line pass, low values encourage line overlap [0, 1]
    #[clap(short, long, value_parser=number_between_0_and_1, default_value_t = 0.5)]
    pub lighten_factor: f64,
    /// Write pegs to file
    #[clap(long, name = "PEG_SAVE_FILE")]
    pub save_pegs: Option<String>,
    /// Read pegs from file
    #[clap(long, name="PEG_LOAD_FILE", validator=check_file_exists)]
    pub load_pegs: Option<String>,
    /// Verbosity level.
    #[clap(flatten)]
    pub verbose: Verbosity,
}
