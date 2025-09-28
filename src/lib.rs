use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hallo, {}! Willkommen bei Rust + WebAssembly 🚀", name)
}
