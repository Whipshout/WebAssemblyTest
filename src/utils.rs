//! Utils for libraries.
//!
//! No #[wasm_bindgen] here
//! Functions here are not export to js
//! They are just to support exported functions

use crate::Provider;
use wasm_bindgen::JsValue;

/// Convert struct to js object option
pub fn get_object(provider: &Provider) -> Option<JsValue> {
    match JsValue::from_serde(provider) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}
