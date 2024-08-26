use image::DynamicImage;
use log::info;
use rayon::prelude::*;
use resvg::render;
use resvg::tiny_skia;
use resvg::usvg;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use svg::node::element::path::Data;
use svg::node::element::{Path as PathSVG, Rectangle};
use svg::{Document, Node};

use crate::peg::{Peg, Yarn};
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
/// A string art [`Blueprint`]. Holds the result of the [`crate::pather::Pather`]'s pathing algorithm and renders it to file.
pub struct Blueprint {
    /// The order with which to connect the [`Pegs`](Peg).
    pub peg_order: Vec<Peg>,
    /// Width of the [`Blueprint`], should be the same dimensions as the image used.
    pub width: u32,
    /// Height of the [`Blueprint`], should be the same dimensions as the image used.
    pub height: u32,
    /// Background color, if None no background is added. It will be transparent for svg and alpha
    /// compatible image formats.
    pub background: Option<(u8, u8, u8)>,
    /// Render scale, how much to up/down scale the render.
    pub render_scale: f64,
    /// Display progress bar.
    #[serde(skip)]
    pub progress_bar: bool,
}

impl Blueprint {
    /// Creates a new [`Blueprint`].
    pub fn new(
        peg_order: Vec<Peg>,
        width: u32,
        height: u32,
        background: Option<(u8, u8, u8)>,
        render_scale: f64,
        progress_bar: bool,
    ) -> Self {
        Self {
            peg_order,
            width,
            height,
            background,
            render_scale,
            progress_bar,
        }
    }

    /// Create a [`Blueprint`] from [`Peg`] references.
    pub fn from_refs(
        peg_order: Vec<&Peg>,
        width: u32,
        height: u32,
        background: Option<(u8, u8, u8)>,
        render_scale: f64,
        progress_bar: bool,
    ) -> Self {
        Self {
            peg_order: peg_order.into_iter().copied().collect(),
            width,
            height,
            background,
            render_scale,
            progress_bar,
        }
    }

    /// Read a [`Blueprint`] from a json file.
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(File::open(file_path)?);
        let out: Self = serde_json::from_reader(reader)?;

        Ok(out)
    }

    /// Write a [`Blueprint`] to a json file.
    pub fn to_file<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        serde_json::to_writer(&file, &self)?;
        Ok(())
    }

    /// Iterate over successive pairs of [`Pegs`](Peg).
    ///
    /// # Examples
    ///
    ///```
    /// use strandify::blueprint::Blueprint;
    /// use strandify::peg::Peg;
    /// let bp = Blueprint::new(vec![Peg::new(0, 0, 0), Peg::new(3, 3, 1)], 4, 4, Some((255, 255, 255)), 1., false);
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
    pub fn render_img(&self, yarn: &Yarn) -> Result<image::RgbaImage, Box<dyn Error>> {
        let document = self.render_svg(yarn)?;
        let svg_data = document.to_string();
        let svg_tree = usvg::Tree::from_str(&svg_data, &usvg::Options::default()).unwrap();

        let render_width = (self.width as f64 * self.render_scale).round() as u32;
        let render_height = (self.height as f64 * self.render_scale).round() as u32;

        // divide the height into chunks to be processed in parallel
        let num_chunks = rayon::current_num_threads();
        let chunk_height = (render_height + num_chunks as u32 - 1) / num_chunks as u32;

        let pbar = utils::spinner(!self.progress_bar).with_message("Rendering image");
        pbar.enable_steady_tick(Duration::from_millis(100));

        // render each chunk in parallel
        let chunks: Vec<tiny_skia::Pixmap> = (0..num_chunks)
            .into_par_iter()
            .map(|i| {
                let start_y = i as u32 * chunk_height;
                let end_y = ((i + 1) as u32 * chunk_height).min(render_height);

                let mut pixmap = tiny_skia::Pixmap::new(render_width, end_y - start_y).unwrap();

                let transform = tiny_skia::Transform::from_translate(0.0, -(start_y as f32));
                render(&svg_tree, transform, &mut pixmap.as_mut());
                pixmap
            })
            .collect();

        pbar.finish_and_clear();

        // create the final image buffer
        let mut final_pixmap = tiny_skia::Pixmap::new(render_width, render_height).unwrap();

        // combine the chunks back into the final image
        for (i, pixmap) in chunks.into_iter().enumerate() {
            let start_y = i as u32 * chunk_height;
            final_pixmap.draw_pixmap(
                0,
                start_y as i32,
                pixmap.as_ref(),
                &tiny_skia::PixmapPaint::default(),
                tiny_skia::Transform::identity(),
                None,
            );
        }

        // convert the final pixmap to an image::RgbaImage
        let img =
            image::ImageBuffer::from_vec(render_width, render_height, final_pixmap.data().to_vec())
                .unwrap();

        Ok(img)
    }

    /// Render the [`Blueprint`] as a svg.
    ///
    /// # Arguments
    ///
    /// * `yarn`: The [`Yarn`] to use to render the [`Blueprint`].
    pub fn render_svg(&self, yarn: &Yarn) -> Result<Document, Box<dyn Error>> {
        let (r, g, b) = yarn.color;
        let render_width = (self.width as f64 * self.render_scale).round() as u32;
        let render_height = (self.height as f64 * self.render_scale).round() as u32;
        info!("Render resolution: {render_width}x{render_height}");

        let mut document = Document::new()
            .set("viewbox", (0, 0, render_width, render_height))
            .set("width", render_width)
            .set("height", render_height);

        if let Some((bg_r, bg_g, bg_b)) = self.background {
            let background = Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", format!("rgb({bg_r}, {bg_g}, {bg_b})"));
            document.append(background);
        }

        let pbar = utils::pbar(self.peg_order.len() as u64 - 1, !self.progress_bar)?
            .with_message("Rendering svg");

        for (peg_a, peg_b) in pbar.wrap_iter(self.zip()) {
            let data = Data::new()
                .move_to((
                    (peg_a.x as f64 * self.render_scale) as u32,
                    (peg_a.y as f64 * self.render_scale) as u32,
                ))
                .line_to((
                    (peg_b.x as f64 * self.render_scale) as u32,
                    (peg_b.y as f64 * self.render_scale) as u32,
                ));
            let path = PathSVG::new()
                .set("fill", "none")
                .set("stroke", format!("rgb({r}, {g}, {b})"))
                .set("stroke-width", yarn.width)
                .set("opacity", yarn.opacity)
                .set("stroke-linecap", "round")
                .set("d", data);
            document.append(path);
        }
        Ok(document)
    }

    /// Render the [`Blueprint`].
    ///
    /// # Arguments:
    ///
    /// * `path`: Output file path, image format or svg.
    /// * `yarn`: The [`Yarn`] to use to render the [`Blueprint`].
    /// * `progress_bar`: Show progress bar.
    pub fn render<P: AsRef<Path>>(&self, path: P, yarn: &Yarn) -> Result<(), Box<dyn Error>> {
        let path = path.as_ref();
        let extension = path.extension().ok_or("Could not detemine extension.")?;
        if extension == "svg" {
            let svg_img = self.render_svg(yarn)?;
            svg::save(path, &svg_img)?;
        } else {
            let img = self.render_img(yarn)?;
            if path.extension().unwrap() != "png" {
                // drop alpha channel
                let out = DynamicImage::from(img).to_rgb8();
                out.save(path)?;
            } else {
                img.save(path)?;
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
            1.,
            true,
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
            1.,
            true,
        );
        assert_eq!(bp.zip().len(), 1);
    }
}
