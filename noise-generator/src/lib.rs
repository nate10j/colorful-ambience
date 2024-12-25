use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
#[wasm_bindgen(js_namespace = console)]
fn log(s: &str);
#[wasm_bindgen(js_namespace = console, js_name = log)]
fn log_u32(a: u32);
}

#[wasm_bindgen]
pub enum ColorNoise {
    White,
    Pink,
    Brown
}

#[wasm_bindgen]
pub struct NoiseGenerator {
    color_noise: ColorNoise,
    i: u32 // for debugging purposes
}

fn fill_white_noise(buffer: &mut [f32]) {
    for a in buffer {
        *a = fastrand::f32() * 2.0 - 1.0;
    }
}

const PINK_NUM_OCTAVES: u32 = 6;

fn fill_pink_noise(buffer: &mut [f32]) {
    for a in buffer {

    }
}

fn fill_brown_noise(buffer: &mut [f32]) {
    for a in buffer {

    }
}

#[wasm_bindgen]
impl NoiseGenerator {
    pub fn new(color_noise: ColorNoise) -> Self {
        Self { i: 0, color_noise }
    }
    pub fn process(&mut self, output: &mut [f32]) -> bool {
        match self.color_noise {
            ColorNoise::White => fill_white_noise(output),
            ColorNoise::Pink => fill_pink_noise(output),
            ColorNoise::Brown => fill_brown_noise(output),
        }

        if self.i % 1000 == 0 {
            log_u32(output.len().try_into().unwrap());
        }
        self.i += 1;

        true
    }
}
