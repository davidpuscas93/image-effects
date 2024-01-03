use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale processing started.".into());

    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();

    log(&"Base64 decoded.".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();

    log(&"Image loaded.".into());

    img = img.grayscale();

    log(&"Grayscale effect applied.".into());

    let mut buffer = vec![];

    img.write_to(&mut buffer, Png).unwrap();

    log(&"Grayscale image buffered.".into());

    let buffer_to_base64 = general_purpose::STANDARD.encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", buffer_to_base64);

    data_url
}
