mod wrapper;
use base64::{engine::general_purpose, Engine as _};
use std::io::Cursor;
use strandify::{blueprint::Blueprint, pather::Pather};
use wasm_bindgen::prelude::*;

fn compute_bp(
    image_data: &[u8],
    pegs: Vec<wrapper::Peg>,
    pather_config: wrapper::PatherConfig,
) -> Result<Blueprint, JsValue> {
    let image = match image::load_from_memory(image_data) {
        Ok(image) => image.to_luma8(),
        Err(err) => return Err(JsValue::from(err.to_string())),
    };

    let pegs = pegs.iter().map(|peg| peg.inner).collect();
    let mut pather = Pather::new(image, pegs, pather_config.inner);

    pather
        .compute()
        .map_err(|err| JsValue::from(err.to_string()))
}

/// Compute the [`Blueprint`](crate::blueprint::Blueprint) of the image and return it as an SVG string.
#[wasm_bindgen]
pub fn compute_svg(
    image_data: &[u8],
    pegs: Vec<wrapper::Peg>,
    pather_config: wrapper::PatherConfig,
    yarn: wrapper::Yarn,
) -> Result<String, JsValue> {
    let bp = compute_bp(image_data, pegs, pather_config)?;

    Ok(bp
        .render_svg(&yarn.inner)
        .map_err(|err| JsValue::from(err.to_string()))?
        .to_string())
}

/// Compute the [`Blueprint`](crate::blueprint::Blueprint) of the image and return it as a base64-encoded PNG string.
#[wasm_bindgen]
pub fn compute_png(
    image_data: &[u8],
    pegs: Vec<wrapper::Peg>,
    pather_config: wrapper::PatherConfig,
    yarn: wrapper::Yarn,
) -> Result<String, JsValue> {
    let bp = compute_bp(image_data, pegs, pather_config)?;
    let img = bp
        .render_img(&yarn.inner)
        .map_err(|err| JsValue::from(err.to_string()))?;

    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), image::ImageFormat::Png)
        .map_err(|err| err.to_string())?;

    Ok(general_purpose::STANDARD.encode(image_data))
}
