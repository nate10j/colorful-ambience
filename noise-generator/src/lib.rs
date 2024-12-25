use wasm_bindgen::prelude::*;

// we need a seperate struct because we need to save seperate values
// and simply putting an object will reset them
mod pink_noise_generator;
use crate::pink_noise_generator::PinkNoiseGenerator;

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

fn fill_white_noise(buffer: &mut [f32]) {
    for a in buffer {
        *a = fastrand::f32() * 2.0 - 1.0;
    }
}

fn fill_brown_noise(buffer: &mut [f32]) {
    for a in buffer {

    }
}

fn fill_pink_noise(buffer: &mut [f32], pink_noise_generator: &mut PinkNoiseGenerator) {
    for a in buffer {
        *a = pink_noise_generator.sample();
    }
}

#[wasm_bindgen]
pub struct NoiseGenerator {
    color_noise: ColorNoise,
    pink_noise_generator: PinkNoiseGenerator,
    i: u32 // debugging purposes
}

#[wasm_bindgen]
impl NoiseGenerator {
    pub fn new(color_noise: ColorNoise) -> Self {
        Self {
            color_noise,
            pink_noise_generator: PinkNoiseGenerator::new(16),
            i: 0
        }
    }
    pub fn process(&mut self, output: &mut [f32]) -> bool {
        match self.color_noise {
            ColorNoise::White => fill_white_noise(output),
            ColorNoise::Pink => fill_pink_noise(output, &mut self.pink_noise_generator),
            ColorNoise::Brown => fill_brown_noise(output),
        }

        if self.i % 1000 == 0 {
            log_u32(output.len().try_into().unwrap());
        }
        self.i += 1;

        true
    }
}
