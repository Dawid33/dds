//#![no_std]
#![feature(lang_items, start)]

//mod std;
//pub use std::{panic};


pub fn add_ones() -> i32 {
    unsafe {
        return add_ones_wasm();
    }
}

extern "C" {
    fn add_ones_wasm() -> i32;
}

