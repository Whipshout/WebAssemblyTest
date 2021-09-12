//! Test suite for node.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_assembly_whipshout::{
    convert_to_provider, parse_string, parse_string_option, send_array, send_vector, Provider,
};

/// Test if function can parse with js values
#[wasm_bindgen_test]
pub fn parse_ok() {
    let n = parse_string("5").unwrap();

    assert_eq!(JsValue::from(5), n);
}

/// Test if function cannot parse with js values
#[wasm_bindgen_test]
pub fn parse_fail() {
    let n = parse_string("asdf").unwrap();

    assert_eq!(JsValue::undefined(), n);
}

/// Test if function can parse with rust values
#[wasm_bindgen_test]
pub fn parse_ok_option() {
    let n = parse_string_option("5");

    assert_eq!(Some(5), n);
}

/// Test if function cannot parse with rust values
#[wasm_bindgen_test]
pub fn parse_fail_option() {
    let n = parse_string_option("asdf");

    assert_eq!(None, n);
}

/// Test if can create, get and change the info using the struct methods
#[wasm_bindgen_test]
pub fn check_provider() {
    let mut provider = Provider::new(123, "Magic".to_owned());
    let id = provider.id();
    let name = provider.name();

    assert_eq!(id, 123);
    assert_eq!(name, "Magic".to_owned());

    provider.set_id(456);
    provider.set_name("JetBrains".to_owned());
    let id = provider.id();
    let name = provider.name();

    assert_eq!(id, 456);
    assert_eq!(name, "JetBrains".to_owned());
}

/// Test if we can convert from Rust struct to JS object
/// It also tests if we can convert from JS object to Rust structure
#[wasm_bindgen_test]
pub fn convert_object_to_struct() {
    let provider = Provider::new(123, "Magic".to_owned());
    let object = provider.get_object();
    let converted_provider = convert_to_provider(&object);

    let (id, name) = if let Some(provider) = converted_provider {
        (provider.id(), provider.name())
    } else {
        (0, "".to_owned())
    };

    assert_eq!(123, id);
    assert_eq!("Magic", name);
}

/// Test if we send the correct vector
#[wasm_bindgen_test]
pub fn send_vector_rust() {
    let vector = vec![1, 2, 3, 4, 5];

    assert_eq!(vector, send_vector());
}

/// Test if we send the correct array
#[wasm_bindgen_test]
pub fn send_vector_js() {
    let numbers = send_array();

    assert!(!JsValue::is_null(&numbers));
}
