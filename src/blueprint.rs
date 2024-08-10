use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::{imageops, DynamicImage, Rgba};
use serde::{Deserialize, Serialize};
use serde_json::Result as Result_serde;
use svg::node::element::path::Data;
use svg::node::element::{Path as PathSVG, Rectangle};
use svg::{Document, Node};

use crate::peg::{Peg, Yarn};
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blueprint {
    /// The order with which to connect the [`Pegs`](Peg).
    pub peg_order: Vec<Peg>,
    /// Width of the [`Blueprint`].
    pub width: u32,
    /// Height of the [`blueprint`].
    pub height: u32,
    /// Background
    pub background: Option<(u8, u8, u8)>,
}

impl Blueprint {
    /// Creates a new [`Blueprint`].
    pub fn new(
        peg_order: Vec<Peg>,
        width: u32,
        height: u32,
        background: Option<(u8, u8, u8)>,
    ) -> Self {
        Self {
            peg_order,
            width,
            height,
            background,
        }
    }

    /// Create a [`Blueprint`] from [`Peg`] references.
    pub fn from_refs(
        peg_order: Vec<&Peg>,
        width: u32,
        height: u32,
        background: Option<(u8, u8, u8)>,
    ) -> Self {
        Self {
            peg_order: peg_order.into_iter().copied().collect(),
            width,
            height,
            background,
        }
    }

    /// Read a [`Blueprint`] from a json file.
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(File::open(file_path)?);
        let out: Self = serde_json::from_reader(reader)?;

        Ok(out)
    }

    /// Write a [`Blueprint`] to a json file.
    pub fn to_file<P: AsRef<Path>>(&self, file_path: P) -> Result_serde<()> {
        let file = File::create(file_path).unwrap();
        serde_json::to_writer(&file, &self)?;

        Ok(())
    }

    /// Iterate over successive pairs of [`Pegs`](Peg).
    ///
    /// # Examples
    ///
    ///```
    /// use stringart::blueprint::Blueprint;
    /// use stringart::peg::Peg;
    /// let bp = Blueprint::new(vec![Peg::new(0, 0, 0), Peg::new(3, 3, 1)], 4, 4);
    /// for (peg_a, peg_b) in bp.zip() {
    ///     assert_eq!(peg_a.id, 0);
    ///     assert_eq!(peg_b.id, 1);
    /// }
    /// assert_eq!(bp.zip().len(), 1);
    ///```
    pub fn zip(
        &self,
    ) -> std::iter::Zip<std::slice::Iter<Peg>, std::iter::Skip<std::slice::Iter<Peg>>> {
        self.peg_order.iter().zip(self.peg_order.iter().skip(1))
    }

    /// Render the [`Blueprint`] as a raster image.
    ///
    /// # Arguments
    ///
    /// * `yarn`: The [`Yarn`] to use to render the [`Blueprint`].
    /// * `progress_bar`: Show progress bar.
    pub fn render_img(&self, yarn: &Yarn, progress_bar: bool) -> image::RgbaImage {
        let (r, g, b) = yarn.color;
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(r, g, b, (yarn.opacity * 255.).round() as u8);
        paint.anti_alias = true;

        let path = {
            let mut pb = tiny_skia::PathBuilder::new();
            let mut iter = self.peg_order.iter();
            let start = iter.next().unwrap();

            pb.move_to(start.x as f32, start.y as f32);
            let pbar = utils::pbar(self.peg_order.len() as u64 - 1, !progress_bar)
                .with_message("Rendering image");
            for peg in pbar.wrap_iter(iter) {
                pb.line_to(peg.x as f32, peg.y as f32);
            }
            pb.finish().unwrap()
        };

        let stroke = tiny_skia::Stroke {
            width: yarn.width as f32 / 2.,
            line_cap: tiny_skia::LineCap::Round,
            ..Default::default()
        };

        let mut pixmap = tiny_skia::Pixmap::new(self.width, self.height).unwrap();
        pixmap.stroke_path(
            &path,
            &paint,
            &stroke,
            tiny_skia::Transform::identity(),
            None,
        );
        // pixmap.save_png("image.png").unwrap();
        let img =
            image::ImageBuffer::from_vec(self.width, self.height, pixmap.data().to_vec()).unwrap();

        if let Some((r, g, b)) = self.background {
            // add a background
            let mut out =
                image::RgbaImage::from_pixel(self.width, self.height, Rgba([r, g, b, 255]));
            imageops::overlay(&mut out, &img, 0, 0);
            out
        } else {
            img
        }
    }

    /// Render the [`Blueprint`] as a svg.
    ///
    /// # Arguments
    ///
    /// * `yarn`: The [`Yarn`] to use to render the [`Blueprint`].
    /// * `progress_bar`: Show progress bar.
    pub fn render_svg(&self, yarn: &Yarn, progress_bar: bool) -> Document {
        let (r, g, b) = yarn.color;
        let mut document = Document::new()
            .set("viewbox", (0, 0, self.width, self.height))
            .set("width", self.width)
            .set("height", self.height);

        let mut background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "100%")
            .set("height", "100%");
        if let Some((bg_r, bg_g, bg_b)) = self.background {
            background = background.set("fill", format!("rgb({bg_r}, {bg_g}, {bg_b})"));
        }
        document.append(background);

        let pbar = utils::pbar(self.peg_order.len() as u64 - 1, !progress_bar)
            .with_message("Rendering svg");

        for (peg_a, peg_b) in pbar.wrap_iter(self.zip()) {
            let data = Data::new()
                .move_to((peg_a.x, peg_a.y))
                .line_to((peg_b.x, peg_b.y));
            let path = PathSVG::new()
                .set("fill", "none")
                .set("stroke", format!("rgb({r}, {g}, {b})"))
                .set("stroke-width", yarn.width)
                .set("opacity", yarn.opacity)
                .set("d", data);
            document.append(path);
        }
        document
    }

    /// Render the [`Blueprint`].
    ///
    /// # Arguments:
    ///
    /// * `output_file`: Output file path, image format or svg.
    /// * `yarn`: The [`Yarn`] to use to render the [`Blueprint`].
    /// * `progress_bar`: Show progress bar.
    pub fn render(
        &self,
        output_file: &Path,
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
            if output_file.extension().unwrap() != "png" {
                let out = DynamicImage::from(img).to_rgb8();
                out.save(output_file)?;
            } else {
                img.save(output_file)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    static TEST_DIR: &str = "./test_blueprint/";

    #[cfg(test)]
    #[ctor::ctor]
    fn setup() {
        let test_dir = PathBuf::from(TEST_DIR);
        fs::create_dir(test_dir).unwrap();
    }

    #[cfg(test)]
    #[ctor::dtor]
    fn teardown() {
        let test_dir = PathBuf::from(TEST_DIR);
        if test_dir.is_dir() {
            fs::remove_dir_all(&test_dir).unwrap();
        }
    }

    #[test]
    fn blueprint_to_from_file() {
        let bp = Blueprint::new(
            vec![Peg::new(0, 0, 0), Peg::new(63, 63, 1)],
            64,
            64,
            Some((0, 0, 0)),
        );
        let bp_file = PathBuf::from(TEST_DIR).join("bp.json");
        assert!(bp.to_file(&bp_file).is_ok());

        let bp_read = Blueprint::from_file(&bp_file).unwrap();
        assert_eq!(bp.height, bp_read.height);
        assert_eq!(bp.width, bp_read.width);
        for (peg_a, peg_b) in bp.peg_order.iter().zip(&bp_read.peg_order) {
            assert_eq!(peg_a.id, peg_b.id);
            assert_eq!(peg_a.x, peg_b.x);
            assert_eq!(peg_a.y, peg_b.y);
        }
    }

    #[test]
    fn zip() {
        let bp = Blueprint::new(
            vec![Peg::new(0, 0, 0), Peg::new(63, 63, 1)],
            64,
            64,
            Some((255, 255, 255)),
        );
        assert_eq!(bp.zip().len(), 1);
    }
}
