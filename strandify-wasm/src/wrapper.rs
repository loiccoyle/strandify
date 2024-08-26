use strandify::pather::EarlyStopConfig as RsEarlyStopConfig;
use strandify::pather::PatherConfig as RsPatherConfig;
use strandify::peg::Peg as RsPeg;
use strandify::peg::Yarn as RsYarn;
use wasm_bindgen::prelude::*;

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

    #[wasm_bindgen]
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
    pub fn new(loss_threshold: Option<f64>, max_count: u32) -> Self {
        Self {
            inner: RsEarlyStopConfig {
                loss_threshold,
                max_count,
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
        early_stop: EarlyStopConfig,
        start_peg_radius: u32,
        skip_peg_within: u32,
        beam_width: usize,
    ) -> Self {
        Self {
            inner: RsPatherConfig::new(
                iterations,
                yarn.inner,
                early_stop.inner,
                start_peg_radius,
                skip_peg_within,
                beam_width,
                false,
            ),
        }
    }
}
