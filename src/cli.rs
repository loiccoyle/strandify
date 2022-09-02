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
    /// Image file for which to generate knit art.
    #[clap(validator=check_file_exists)]
    pub image: String,
    #[clap(flatten)]
    /// Verbosity level.
    pub verbose: Verbosity,
}
