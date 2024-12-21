use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! rand: {}", name, fastrand::i32(..)));
}

pub fn white_noise() {
    alert("generating white noise");
}
