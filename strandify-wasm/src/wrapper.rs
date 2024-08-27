#![allow(non_snake_case)]
use strandify::pather::EarlyStopConfig as RsEarlyStopConfig;
use strandify::pather::PatherConfig as RsPatherConfig;
use strandify::peg::Peg as RsPeg;
use strandify::peg::Yarn as RsYarn;
use wasm_bindgen::prelude::*;

use strandify::utils::circle_coords as rs_circle_coords;
use strandify::utils::line_coords as rs_line_coords;
use strandify::utils::rectangle_coords as rs_rectangle_coords;
use strandify::utils::square_coords as rs_square_coords;

#[derive(Clone)]
#[wasm_bindgen]
pub struct ShapeCoords {
    x: Vec<u32>,
    y: Vec<u32>,
}

#[wasm_bindgen]
impl ShapeCoords {
    #[wasm_bindgen]
    pub fn get_x(&self) -> Vec<u32> {
        self.x.clone()
    }
    #[wasm_bindgen]
    pub fn get_y(&self) -> Vec<u32> {
        self.y.clone()
    }
}

#[wasm_bindgen(js_name = circleCoords)]
pub fn circle_coords(x: u32, y: u32, radius: u32, n_points: usize) -> ShapeCoords {
    let (x, y) = rs_circle_coords((x, y), radius, n_points);
    ShapeCoords { x, y }
}
#[wasm_bindgen(js_name = lineCoords)]
pub fn line_coords(x1: u32, y1: u32, x2: u32, y2: u32, n_points: usize) -> ShapeCoords {
    let (x, y) = rs_line_coords((x1, y1), (x2, y2), n_points);
    ShapeCoords { x, y }
}
#[wasm_bindgen(js_name = squareCoords)]
pub fn square_coords(x: u32, y: u32, width: u32, n_points: usize) -> ShapeCoords {
    let (x, y) = rs_square_coords((x, y), width, n_points);
    ShapeCoords { x, y }
}
#[wasm_bindgen(js_name = rectangleCoords)]
pub fn rectangle_coords(x: u32, y: u32, width: u32, height: u32, n_points: usize) -> ShapeCoords {
    let (x, y) = rs_rectangle_coords((x, y), width, height, n_points);
    ShapeCoords { x, y }
}

#[wasm_bindgen]
pub struct Peg {
    pub(crate) inner: RsPeg,
}

#[wasm_bindgen]
impl Peg {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            inner: RsPeg::new(x, y),
        }
    }

    #[wasm_bindgen(js_name = withJitter)]
    pub fn with_jitter(&self, jitter: i64) -> Self {
        Self {
            inner: self.inner.with_jitter(jitter),
        }
    }
}

#[wasm_bindgen]
pub struct Yarn {
    pub(crate) inner: RsYarn,
}

#[wasm_bindgen]
impl Yarn {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, opacity: f64, r: u8, g: u8, b: u8) -> Self {
        Self {
            inner: RsYarn::new(width, opacity, (r, g, b)),
        }
    }
}

#[wasm_bindgen]
pub struct EarlyStopConfig {
    pub(crate) inner: RsEarlyStopConfig,
}

#[wasm_bindgen]
impl EarlyStopConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(lossThreshold: Option<f64>, maxCount: u32) -> Self {
        Self {
            inner: RsEarlyStopConfig {
                loss_threshold: lossThreshold,
                max_count: maxCount,
            },
        }
    }
}

#[wasm_bindgen]
pub struct PatherConfig {
    pub(crate) inner: RsPatherConfig,
}

#[wasm_bindgen]
impl PatherConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(
        iterations: usize,
        yarn: Yarn,
        earlyStop: EarlyStopConfig,
        startPegRadius: u32,
        skipPegWithin: u32,
        beamWidth: usize,
    ) -> Self {
        Self {
            inner: RsPatherConfig::new(
                iterations,
                yarn.inner,
                earlyStop.inner,
                startPegRadius,
                skipPegWithin,
                beamWidth,
                false,
            ),
        }
    }
}
