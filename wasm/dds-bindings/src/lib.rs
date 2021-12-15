#![no_std]
#![no_main]
#![feature(lang_items,start)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(used)]

mod std;

#[start]
#[lang = "start"]
fn start<T>(main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
where
    T: Termination,
{
    main();

    0
}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

pub fn hello_world() {
    unsafe {
        return hello_world_wasm();
    }
}

extern "C" {
    fn hello_world_wasm();
}