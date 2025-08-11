use wasm_bindgen::prelude::*;

// Re-export the WASM interface
pub mod wasm;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
