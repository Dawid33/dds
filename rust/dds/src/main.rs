#![allow(unused)]
use serde::{Serialize, Deserialize};

mod exports;
use exports::*;

use wasmi::{
    Error, Externals, FuncInstance, FuncRef, HostError, ImportsBuilder,
    ModuleImportResolver, ModuleInstance, ModuleRef, RuntimeArgs, RuntimeValue, Signature, Trap,
    ValueType, NopExternals
};

pub struct MyType {
    pub name : String,
}

const WASM_PATH : &'static str = "../wasm/target/wasm32-unknown-unknown/debug/dds-example.wasm";

struct RuntimeModuleImportResolver;

const HELLO_WORLD_WASM_INDEX: usize = 0 ;

impl<'a> ModuleImportResolver for RuntimeModuleImportResolver {
    fn resolve_func(&self, field_name: &str, _signature: &wasmi::Signature) -> Result<wasmi::FuncRef, wasmi::Error> {
        let func_ref = match field_name {
            "hello_world_wasm" => FuncInstance::alloc_host(
        Signature::new(&[][..], None),
                HELLO_WORLD_WASM_INDEX,
            ),
            _ => {
                return Err(Error::Function(format!(
                    "host module doesn't export function with name {}",
                    field_name
                )));
            }
        };
        Ok(func_ref)
    }
}

struct Runtime;

impl Externals for Runtime {
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            HELLO_WORLD_WASM_INDEX => {
                hello_world();
                Ok(None)
            },
            _ => panic!("unknown function index")
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bin = std::fs::read(WASM_PATH)?;
    let module  = wasmi::Module::from_buffer(&wasm_bin).unwrap();
    let mut imports = ImportsBuilder::new();
    imports.push_resolver("env", &RuntimeModuleImportResolver);
    let mut runtime = Runtime;
    let instance = ModuleInstance::new(&module, &imports).unwrap().assert_no_start();
    let result_opt = instance.invoke_export::<Runtime>("main", &[RuntimeValue::I32(1),RuntimeValue::I32(1)], &mut runtime).unwrap();
    
    Ok(())
}
