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
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Provider struct to test data conversion
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
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
        use web_sys::console;

        console::log_1(&"Created new structure".into());
        console::log_2(&"Id:".into(), &JsValue::from(id));

        Self { id, name }
    }

    /// Returns id
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Changes id
    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    /// Returns name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Changes name
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    /// Get as JS Object
    #[wasm_bindgen]
    pub fn get_object(&self) -> JsValue {
        if let Some(val) = get_object(self) {
            val
        } else {
            JsValue::undefined()
        }
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

/// Convert JS Object to Provider struct
#[wasm_bindgen]
pub fn convert_to_provider(val: &JsValue) -> Option<Provider> {
    match val.into_serde() {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

/// Send a Rust vector to JS
#[wasm_bindgen]
pub fn send_vector() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

/// Send a JS Array
#[wasm_bindgen]
pub fn send_array() -> js_sys::Array {
    let nums = js_sys::Array::new();
    nums.push(&JsValue::from(1));
    nums.push(&JsValue::from(2));
    nums.push(&JsValue::from(3));
    nums.push(&JsValue::from(4));
    nums.push(&JsValue::from(5));

    nums
}

/// Get a JS array and returns it with values multiplied by 2
#[wasm_bindgen]
pub fn return_array_modified(iter: &JsValue) -> Result<js_sys::Array, JsValue> {
    let nums = js_sys::Array::new();

    let iterator = js_sys::try_iter(iter)?.ok_or("need to pass iterable JS values!")?;

    let numbers = iterator
        .map(|x| {
            if let Ok(x) = x {
                x * JsValue::from(2)
            } else {
                JsValue::undefined()
            }
        })
        .collect::<Vec<JsValue>>();

    for n in numbers.iter() {
        nums.push(n);
    }

    Ok(nums)
}
