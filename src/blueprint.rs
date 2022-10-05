use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use image::GrayImage;
use serde::{Deserialize, Serialize};
use serde_json::Result as Result_serde;
use svg::node::element::path::Data;
use svg::node::element::{Path as PathSVG, Rectangle};
use svg::{Document, Node};

use crate::peg::{Peg, Yarn};
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blueprint {
    pub peg_order: Vec<Peg>,
    pub width: u32,
    pub height: u32,
}

impl Blueprint {
    pub fn new(peg_order: Vec<Peg>, width: u32, height: u32) -> Self {
        Self {
            peg_order,
            width,
            height,
        }
    }

    /// Create a blueprint from `Peg` references.
    pub fn from_refs(peg_order: Vec<&Peg>, width: u32, height: u32) -> Self {
        Self {
            peg_order: peg_order.into_iter().copied().collect(),
            width,
            height,
        }
    }

    /// Read a blueprint from a json file.
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(File::open(file_path)?);
        let out: Self = serde_json::from_reader(reader)?;

        Ok(out)
    }

    /// Write a blueprint to a json file.
    pub fn to_file<P: AsRef<Path>>(&self, file_path: P) -> Result_serde<()> {
        let file = File::create(file_path).unwrap();
        serde_json::to_writer(&file, &self)?;

        Ok(())
    }

    /// Iterate over successive pairs of pegs.
    pub fn zip(
        &self,
    ) -> std::iter::Zip<std::slice::Iter<Peg>, std::iter::Skip<std::slice::Iter<Peg>>> {
        self.peg_order.iter().zip(self.peg_order.iter().skip(1))
    }

    /// Render the blueprint as a raster image.
    ///
    /// # Arguments
    ///
    /// * `yarn`- The yarn to use to render the img.
    /// * `progress_bar`- Show progress bar.
    pub fn render_img(&self, yarn: &Yarn, progress_bar: bool) -> GrayImage {
        let mut img =
            image::GrayImage::from_pixel(self.width, self.height, image::Luma { 0: [255] });

        let opacity = 1. - yarn.opacity;

        let pbar = utils::pbar(self.peg_order.len() as u64 - 1, !progress_bar)
            .with_message("Rendering image");

        // Iterate with pairs of consecutive pegs
        for (peg_a, peg_b) in pbar.wrap_iter(self.zip()) {
            let line = peg_a.line_to(peg_b).with_width(yarn.width);
            line.zip().for_each(|(x, y)| {
                let mut pixel = img.get_pixel_mut(*x, *y);
                // pixel.0[0] = (pixel.0[0] as f64 * 0.99).floor() as u8;
                pixel.0[0] = (pixel.0[0] as f64 * opacity).round() as u8;
            })
        }
        img
    }

    /// Render the blueprint as a svg.
    ///
    /// # Arguments
    ///
    /// * `yarn`- The yarn to use to render the img.
    /// * `progress_bar`- Show progress bar.
    pub fn render_svg(&self, yarn: &Yarn, progress_bar: bool) -> Document {
        let mut document = Document::new()
            .set("viewbox", (0, 0, self.width, self.height))
            .set("width", self.width)
            .set("height", self.height);

        let background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "white");
        document.append(background);

        let pbar = utils::pbar(self.peg_order.len() as u64 - 1, !progress_bar)
            .with_message("Rendering svg");

        for (peg_a, peg_b) in pbar.wrap_iter(self.zip()) {
            let data = Data::new()
                .move_to((peg_a.x, peg_a.y))
                .line_to((peg_b.x, peg_b.y));
            let path = PathSVG::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", yarn.width)
                .set("opacity", yarn.opacity)
                .set("d", data);
            document.append(path);
        }
        document
    }

    /// Render the blueprint
    ///
    /// # Arguments:
    ///
    /// * `output_file`- Output file path.
    /// * `yarn`- The yarn to use to render the img.
    /// * `progress_bar`- Controls the display of the progress bar.
    pub fn render(
        &self,
        output_file: &PathBuf,
        yarn: &Yarn,
        progress_bar: bool,
    ) -> Result<(), Box<dyn Error>> {
        let extension = match output_file.extension() {
            Some(ext) => ext,
            None => return Err("Could not determine file extension".into()),
        };
        if extension == "svg" {
            let svg_img = self.render_svg(yarn, progress_bar);
            svg::save(output_file, &svg_img)?;
        } else {
            let img = self.render_img(yarn, progress_bar);
            img.save(output_file)?;
        }

        Ok(())
    }
}
