mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("henlo")
}

#[wasm_bindgen]
pub fn yeet() -> u32 {
    console::log_1(&format!("yeeeeeeeeeeeet").into());
    4 + 4
}
