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
    #[clap(short, long, value_parser, default_value_t = 200)]
    pub pegs: u32,
    /// Radius scale [0, 1]
    #[clap(short='p', long, value_parser, default_value_t = 0.95)]
    pub peg_radius_scale: f64,
    /// Add angular jitter to the pegs, in rad default: [2*pi/PEGS*5]
    #[clap(short='j', value_parser, long)]
    pub peg_jitter: Option<f64>,
    /// Don't connect neighbouring pegs default: [PEGS/20]
    #[clap(short='n', value_parser, long)]
    pub peg_exclude_neighbours: Option<u16>,
    /// Yarn opacity [0, 1]
    #[clap(short, long, value_parser, default_value_t = 0.02)]
    pub opacity: f32,
    /// Encourages peg exploration at the expense of contrast, should be greater than 1
    #[clap(short, long, value_parser, default_value_t = 1.05)]
    pub lighten_factor: f64,
    /// Verbosity level.
    #[clap(flatten)]
    pub verbose: Verbosity,
}
