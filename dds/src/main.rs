use wasmtime::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let wat = r#"
    (module
        (import "host" "hello" (func $host_hello (param i32)))

        (func (export "hello")
        i32.const 3
        call $host_hello)
    )
    "#;
    let module = Module::new(&engine, wat)?;
    
    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = Store::new(&engine, 4);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, u32>, param: i32| {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
    });
    
    // Instantiation of a module requires specifying its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    let instance = Instance::new(&mut store, &module, &[host_hello.into()])?;
    let hello = instance.get_typed_func::<(), (), _>(&mut store, "hello")?;
    
    // And finally we can call the wasm!
    hello.call(&mut store, ())?;
    Ok(())
}