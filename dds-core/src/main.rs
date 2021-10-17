use wasmer::{Instance, Module, NativeFunc, Store, WasmError, WasmTypeList, imports};

pub struct MyType {
    pub name : String,
}

const WASM_PATH : &'static str = "target/wasm32-unknown-unknown/debug/test.wasm";

fn main() -> anyhow::Result<()> {
    let module_wat = std::fs::read(WASM_PATH)?;
    let store = Store::default();
    let module = Module::new(&store, module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    //let add_one : NativeFunc<(), MyType> = instance.exports.get_native_function("main")?;
    //let result = add_one.call(&[])?;
    //result[0].unwrap_externref();
    //println!("{}", result[0].to_string());
    Ok(())
}
