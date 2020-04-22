mod utils;

use js_sys;
use tokenizations;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn as_vecstring(s: js_sys::Array) -> Vec<String> {
    s.iter().map(|v| v.as_string().unwrap()).collect::<Vec<_>>()
}

#[wasm_bindgen]
pub fn get_alignment(s: js_sys::Array, t: js_sys::Array) -> JsValue {
    let s = as_vecstring(s);
    let t = as_vecstring(t);
    let ret = tokenizations::get_alignments(&s, &t);
    JsValue::from_serde(&ret).unwrap()
}
