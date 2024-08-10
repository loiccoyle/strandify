use clap::Parser;
use clap_verbosity_flag::Verbosity;
use image::ImageReader;
use std::path::PathBuf;
use std::str::FromStr;

fn check_file_exists(input: &str) -> Result<String, String> {
    let input_file = PathBuf::from(input);
    if input_file.exists() {
        let reader = ImageReader::open(&input_file)
            .unwrap()
            .with_guessed_format()
            .unwrap();
        if input_file.extension().unwrap() == "json" || reader.format().is_some() {
            Ok(input.into())
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

#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Rgb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Expected three values separated by spaces".into());
        }

        let r = parts[0].trim().parse().map_err(|_| "Invalid red value")?;
        let g = parts[1].trim().parse().map_err(|_| "Invalid green value")?;
        let b = parts[2].trim().parse().map_err(|_| "Invalid blue value")?;

        Ok(Rgb { r, g, b })
    }
}

#[derive(Parser, Debug)]
#[clap(author = "Loic Coyle")]
/// CLI utility to generate string art.
pub struct Arguments {
    /// Input image or blueprint json file
    #[clap(value_parser=check_file_exists)]
    pub input: String,
    /// Output file, either image format or json
    pub output: Option<String>,
    /// Number of iterations
    #[clap(short, long, value_parser, default_value_t = 4000)]
    pub iterations: u32,
    /// Transparent background
    #[clap(short = 't', action, default_value_t = false)]
    pub transparent: bool,
    /// Yarn color
    #[clap(short = 'C', long, value_parser, default_value = "0 0 0")]
    pub yarn_color: Rgb,
    /// Project image to yarn color.
    #[clap(long, value_parser, default_value_t = false)]
    pub project_to_yarn_color: bool,
    /// Peg distribution shape
    #[clap(short = 'S', long, value_parser=["circle", "square", "border"], default_value = "circle", name="SHAPE")]
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
    #[clap(short = 'w', long, value_parser, default_value_t = 1.)]
    pub yarn_width: f32,
    /// How much to lighten the pixels at each line pass, low values encourage line overlap [0, 1]
    #[clap(short, long, value_parser=number_between_0_and_1, default_value_t = 0.5)]
    pub lighten_factor: f64,
    /// Write pegs to file
    #[clap(long, name = "PEG_SAVE_FILE")]
    pub save_pegs: Option<String>,
    /// Read pegs from file
    #[clap(long, name="PEG_LOAD_FILE", value_parser=check_file_exists)]
    pub load_pegs: Option<String>,
    /// Verbosity level.
    #[clap(flatten)]
    pub verbose: Verbosity,
}
