#![no_std]
#![no_main]
pub use ddsapi::*;

#[no_mangle]
pub fn main() {
    ddsapi::hello_world();

}