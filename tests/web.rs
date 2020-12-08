//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate chardetng_wasm;
use chardetng_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

const ASCII_BYTES: &[u8] = "speak friend and enter".as_bytes();

// "Hello, world." in Russian ("Привет, мир!") encoded via windows-1251
const CYRILLIC_WINDOWS_1251: &[u8] = &[207, 240, 232, 226, 229, 242, 44, 32, 236, 232, 240, 33];

#[wasm_bindgen_test]
fn it_should_detect_empty_input_as_utf8() {
    let result = detect(&[]);
    assert_eq!(result, "UTF-8");
}

#[wasm_bindgen_test]
fn it_should_detect_ascii_as_utf8() {
    let result = detect(ASCII_BYTES);
    assert_eq!(result, "UTF-8");
}

#[wasm_bindgen_test]
fn it_should_detect_windows1251() {
    let result = detect(CYRILLIC_WINDOWS_1251);
    assert_eq!(result, "windows-1251");
}


#[wasm_bindgen_test]
fn it_should_make_a_new_detector() {
    // just ensuring no panics
    let _detector = EncodingDetector::new();
}

#[wasm_bindgen_test]
fn it_should_feed_empty_byte_arrays() {
    let mut detector = EncodingDetector::new();
    // expect false because no non-ascii so far
    assert_eq!(detector.feed(&[], None), false);
}

#[wasm_bindgen_test]
fn it_should_feed_ascii() {
    let mut detector = EncodingDetector::new();
    // expect false because no non-ascii so far
    assert_eq!(detector.feed(ASCII_BYTES, None), false);
}

#[wasm_bindgen_test]
fn it_should_feed_last() {
    let mut detector = EncodingDetector::new();
    detector.feed("Example:".as_bytes(), None);
    assert_eq!(detector.guess(None, Some(true)), "UTF-8");

    detector.feed(CYRILLIC_WINDOWS_1251, Some(true));
    assert_eq!(detector.guess(None, Some(true)), "windows-1251");
}

#[wasm_bindgen_test]
fn it_should_feed_partials() {
    // TODO: test feeding up to the middle of a UTF-8 grapheme and then feeding to the end of the
    // file
}
