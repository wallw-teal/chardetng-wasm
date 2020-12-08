mod utils;

use wasm_bindgen::prelude::*;
use chardetng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn detect(buffer: &[u8]) -> String {
    let mut detector = chardetng::EncodingDetector::new();

    let is_last = false;
    detector.feed(buffer, is_last);

    let top_level_domain = None;
    let allow_utf8 = true;
    detector.guess(top_level_domain, allow_utf8).name().to_string()
}


#[wasm_bindgen]
pub struct EncodingDetector {
    detector: chardetng::EncodingDetector
}

#[wasm_bindgen]
impl EncodingDetector {
    pub fn new() -> Self {
        EncodingDetector {
            detector: chardetng::EncodingDetector::new()
        }
    }

    pub fn feed(&mut self, buffer: &[u8], is_last: Option<bool>) -> bool {
        let last = match is_last {
            None => false,
            Some(last) => last,
        };

        self.detector.feed(buffer, last)
    }

    pub fn guess(&self, top_level_domain: Option<String>, allow_utf8: Option<bool>) -> String {
        let tld = match top_level_domain {
            None => None,
            Some(ref t) => Some(t.as_bytes())
        };

        let utf8 = match allow_utf8 {
            None => false,
            Some(val) => val
        };

        self.detector.guess(tld, utf8).name().to_string()
    }
}
