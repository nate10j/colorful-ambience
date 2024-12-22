use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
pub fn alert(s: &str);
}


#[wasm_bindgen]
pub struct NoiseGenerator {
}

#[wasm_bindgen]
impl NoiseGenerator {
    pub fn new() -> Self {
        Self {}
    }
    pub fn process(&mut self, output: &mut [f32]) -> bool {
        for a in output {
            *a = fastrand::f32() * 2.0 - 1.0;
        }

        true
    }
}
