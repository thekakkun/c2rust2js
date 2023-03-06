#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

pub mod bindings {
    include!("../bindings.rs");
    // #ASK: This won't run. Structs exist, but not functions. Why?
    // include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[wasm_bindgen]
pub fn hello() -> i32 {
    unsafe { bindings::hello() }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    unsafe { bindings::add(a, b) }
}
