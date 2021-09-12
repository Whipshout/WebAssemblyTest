//! Test suite for node.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_assembly_whipshout::{parse_string, parse_string_option, Provider};

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

/// Test if can create and get the info using the struct methods
#[wasm_bindgen_test]
pub fn check_provider() {
    let provider = Provider::new(123, "Klarna".to_owned());
    let id = provider.get_id();
    let name = provider.get_name();

    assert_eq!(id, 123);
    assert_eq!(name, "Klarna".to_owned());
}
