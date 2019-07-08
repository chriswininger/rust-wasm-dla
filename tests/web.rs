//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_rust_dla::DLAField;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn get_ndx_shouldReturnTheCorrectIndex() {
    let ndx = DLAField::get_ndx(4, 4, 600);
    let ndx2 = DLAField::get_ndx(251, 89, 600);
    assert_eq!(ndx, 2404);
    assert_eq!(ndx2, 150689);
}

#[wasm_bindgen_test]
fn new_shouldReturnANewFiled() {
    let field = DLAField::new(60000, 600, 600);
}

#[wasm_bindgen_test]
fn nextState_shouldNotError() {
    let mut field = DLAField::new(60000, 600, 600);
    field.nextState();
}
