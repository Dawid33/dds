#![no_std]
#![no_main]
pub use dds_rust_api;

#[allow(unused)]
#[no_mangle]
pub extern "C" fn dds_start() {
    dds_rust_api::add_ones();
}
