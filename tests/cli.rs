use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use assert_cmd;
use assert_fs::prelude::*;
use image::{self, GenericImageView};
use predicates::prelude::*;
use serde_json;

use stringart::blueprint::Blueprint;
use stringart::peg::Peg;

fn input_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("input.jpg")
}

#[test]
fn test_help() -> Result<(), Box<dyn Error>> {
    let mut cmd = assert_cmd::Command::cargo_bin("stringart")?;
    cmd.arg("--help");

    cmd.assert().success();
    Ok(())
}

#[test]
fn test_save_load_pegs() -> Result<(), Box<dyn Error>> {
    let n_pegs = 10;
    let peg_file = assert_fs::NamedTempFile::new("pegs.json").unwrap();
    let peg_file_str = peg_file.path().to_str().unwrap();

    // write the pegs to file
    let mut cmd = assert_cmd::Command::cargo_bin("stringart")?;
    cmd.arg(input_file().to_str().unwrap());
    cmd.arg("--save-pegs");
    cmd.arg(peg_file_str);
    cmd.arg("--peg-number");
    cmd.arg(n_pegs.to_string());
    cmd.arg("-q");

    cmd.assert().success();
    peg_file.assert(predicate::path::is_file());
    let reader = BufReader::new(File::open(peg_file.path())?);
    let pegs: Vec<Peg> = serde_json::from_reader(reader)?;
    assert_eq!(pegs.len(), n_pegs);

    // read the pegs from file
    let output_file = assert_fs::NamedTempFile::new("output.jpg").unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("stringart")?;
    cmd.arg(input_file().to_str().unwrap());
    cmd.arg("--load-pegs");
    cmd.arg(peg_file_str);
    cmd.arg("-q");
    cmd.arg(output_file.to_str().unwrap());

    cmd.assert().success();
    output_file.assert(predicate::path::is_file());
    let input_img = image::open(input_file())?;
    let output_img = image::open(output_file)?;
    assert_eq!(input_img.dimensions(), output_img.dimensions());
    Ok(())
}

#[test]
fn test_string_art_img() -> Result<(), Box<dyn Error>> {
    let output_file = assert_fs::NamedTempFile::new("output.jpg").unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("stringart")?;
    cmd.arg(input_file().to_str().unwrap());
    cmd.arg(output_file.to_str().unwrap());
    cmd.arg("-i");
    cmd.arg("1000");
    cmd.arg("-q");

    cmd.assert().success();
    output_file.assert(predicate::path::is_file());
    let input_img = image::open(input_file())?;
    let output_img = image::open(output_file)?;
    assert_eq!(input_img.dimensions(), output_img.dimensions());
    Ok(())
}

#[test]
fn test_string_art_svg() -> Result<(), Box<dyn Error>> {
    let output_file = assert_fs::NamedTempFile::new("output.svg").unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("stringart")?;
    cmd.arg(input_file().to_str().unwrap());
    cmd.arg(output_file.to_str().unwrap());
    cmd.arg("-i");
    cmd.arg("1000");
    cmd.arg("-q");

    cmd.assert().success();
    output_file.assert(predicate::path::is_file());
    Ok(())
}

#[test]
fn test_save_render_string_art_blueprint() -> Result<(), Box<dyn Error>> {
    let n_lines = 2000;
    let blueprint_file = assert_fs::NamedTempFile::new("bp.json").unwrap();

    // write blueprint file
    let mut cmd = assert_cmd::Command::cargo_bin("stringart")?;
    cmd.arg(input_file().to_str().unwrap());
    cmd.arg("-i");
    cmd.arg(n_lines.to_string());
    cmd.arg(blueprint_file.to_str().unwrap());
    cmd.arg("-q");

    cmd.assert().success();
    blueprint_file.assert(predicate::path::is_file());
    let input_img = image::open(input_file())?;
    let reader = BufReader::new(File::open(blueprint_file.path())?);
    let bp: Blueprint = serde_json::from_reader(reader)?;
    assert_eq!(input_img.height(), bp.height);
    assert_eq!(input_img.width(), bp.width);
    assert_eq!(bp.peg_order.len(), n_lines + 1);

    // load blueprint file and render img
    let img_file = assert_fs::NamedTempFile::new("bp.jpg").unwrap();
    let mut cmd = assert_cmd::Command::cargo_bin("stringart")?;
    cmd.arg(blueprint_file.to_str().unwrap());
    cmd.arg(img_file.to_str().unwrap());
    cmd.arg("-q");

    cmd.assert().success();
    img_file.assert(predicate::path::is_file());
    Ok(())
}
