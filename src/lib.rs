//! WebAssembly.
//!
//! Learning the idiomatic way to use it.
//! First approach by comparing parse functions and fibonacci.
//!
//! [`WebAssembly Doc`]
//!
//! [`WebAssembly Doc`]: https://rustwasm.github.io/docs/book/introduction.html

mod utils;

use crate::utils::*;
use wasm_bindgen::prelude::*;

/// Provider struct to test data conversion
#[wasm_bindgen]
pub struct Provider {
    id: i32,
    name: String,
}

/// Struct methods to get data in js
#[wasm_bindgen]
impl Provider {
    /// Create a new struct with id and name
    #[wasm_bindgen]
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    /// Returns id
    #[wasm_bindgen]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Returns name
    #[wasm_bindgen]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Placedholder for utils
    #[wasm_bindgen]
    pub fn placeholder_nothing() {
        placeholder();
    }
}

/// Just to test performance in js
#[wasm_bindgen]
pub fn fibonacci(x: i32) -> f64 {
    if x == 0 {
        return 0_f64;
    }

    if x == 1 {
        return 1_f64;
    }

    fibonacci(x - 1) + fibonacci(x - 2)
}

/// Parse int and returns js values
#[wasm_bindgen]
pub fn parse_string(x: &str) -> Result<JsValue, JsValue> {
    match x.parse::<i32>() {
        Ok(n) => Ok(JsValue::from(n)),
        Err(_) => Ok(JsValue::undefined()),
    }
}

/// Parse int and returns rust values (idiomatic way)
#[wasm_bindgen]
pub fn parse_string_option(x: &str) -> Option<i32> {
    match x.parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}
