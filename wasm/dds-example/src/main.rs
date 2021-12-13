#![no_std]
#![no_main]
pub use ddsapp::*;

#[no_mangle]
pub fn main() {
    ddsapp::hello_world();

}