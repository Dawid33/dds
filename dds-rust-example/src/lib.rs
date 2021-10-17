use dds_wasm_api::MyType;

/*
#[repr(C)]
pub enum UI {
    Fltk
}
*/

pub extern fn main() -> MyType {
    MyType {
        name : String::from("Hello"),
    }
}