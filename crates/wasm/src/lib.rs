use wasm_bindgen::prelude::*;

mod primitives;
mod svg;
mod utils;

pub use primitives::*;
pub use svg::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}
