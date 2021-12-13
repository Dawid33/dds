#![no_std]
#![no_main]
#![feature(lang_items, start)]

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

extern "Rust" {
    pub fn main();
}

#[no_mangle]
fn dds_start() -> isize {
    unsafe {
        main();
    }
    0
}

pub fn hello_world() {
    unsafe {
        return hello_world_wasm();
    }
}

extern "C" {
    fn hello_world_wasm();
}