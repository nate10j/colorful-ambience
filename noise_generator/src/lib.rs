use wasm_bindgen::prelude::*;

// we need a seperate struct because we need to save seperate values
// and simply putting an object will reset them
mod pink_noise_generator;
use crate::pink_noise_generator::PinkNoiseGenerator;

#[wasm_bindgen]
pub enum ColorNoise {
    White,
    Pink,
    Brown
}

const WHITE_NOISE_VOLUME_MULTIPLIER: f32 = 0.2;

fn fill_white_noise(buffer: &mut [f32]) {
    for a in buffer {
        *a = (fastrand::f32() * 2.0 - 1.0) * WHITE_NOISE_VOLUME_MULTIPLIER; // scaling
    }
}

fn fill_brown_noise(buffer: &mut [f32], smooth: f32) {
    let mut last_out: f32 = 0.0;
    let gain = 1.0 / (buffer.len() as f32).sqrt();
    for a in buffer {
        let white = fastrand::f32() * 2.0 - 1.0;
        last_out = white + last_out * smooth; // make brown noise sound smoother
        *a = last_out * gain * 0.5; // adjust for gain
    }
}

fn fill_pink_noise(buffer: &mut [f32], pink_noise_generator: &mut PinkNoiseGenerator) {
    for a in buffer {
        *a = pink_noise_generator.sample() * 0.9; // adjust for gain
    }
}

#[wasm_bindgen]
pub struct NoiseGenerator {
    color_noise: ColorNoise,
    pink_noise_generator: PinkNoiseGenerator,
}

#[wasm_bindgen]
impl NoiseGenerator {
    pub fn new(color_noise: ColorNoise) -> Self {
        Self {
            color_noise,
            pink_noise_generator: PinkNoiseGenerator::new(20),
        }
    }
    pub fn process(&mut self, output: &mut [f32]) -> bool {
        match self.color_noise {
            ColorNoise::White => fill_white_noise(output),
            ColorNoise::Pink => fill_pink_noise(output, &mut self.pink_noise_generator),
            ColorNoise::Brown => fill_brown_noise(output, 0.96),
        }

        true
    }

    pub fn change_color(&mut self, color: ColorNoise) {
        self.color_noise = color;
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Write};
    use crate::{ PinkNoiseGenerator, fill_white_noise, fill_brown_noise, fill_pink_noise };

    #[test]
    fn white_noise_samples() -> io::Result<()> {
        let mut file = File::create("white_noise_samples.txt")?;
        let mut noise_array = vec![0.0; 5000];
        fill_white_noise(&mut noise_array);
        for a in noise_array {
            writeln!(file, "{}", a)?; 
        }

        Ok(())
    }

    #[test]
    fn brown_noise_samples() -> io::Result<()> {
        let mut file = File::create("brown_noise_samples.txt")?;
        let mut noise_array = vec![0.0; 5000];
        fill_brown_noise(&mut noise_array, 1.0);
        // for accurate graphing
        // smoothing is only applied for practical purposes
        for a in noise_array {
            writeln!(file, "{}", a)?; 
        }

        Ok(())
    }

    #[test]
    fn pink_noise_samples() -> io::Result<()> {
        let mut file = File::create("pink_noise_samples.txt")?;
        let mut pink_noise_generator = PinkNoiseGenerator::new(20);
        let mut noise_array = vec![0.0; 5000];
        fill_pink_noise(&mut noise_array, &mut pink_noise_generator);
        for a in noise_array {
            writeln!(file, "{}", a)?; 
        }

        Ok(())
    }
}
