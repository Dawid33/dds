use wasmi::{
    Error, Externals, FuncInstance, FuncRef, HostError, ImportsBuilder,
    ModuleImportResolver, ModuleInstance, ModuleRef, RuntimeArgs, RuntimeValue, Signature, Trap,
    ValueType, NopExternals
};

pub struct MyType {
    pub name : String,
}

const WASM_PATH : &'static str = "target/wasm32-unknown-unknown/debug/dds-rust-example.wasm";

pub fn add_ones_wasm() -> i32 {
    println!("Hello");
    4
}

struct RuntimeModuleImportResolver;

const ADD_ONE_WASM_INDEX: usize = 0 ;

impl<'a> ModuleImportResolver for RuntimeModuleImportResolver {
    fn resolve_func(&self, field_name: &str, _signature: &wasmi::Signature) -> Result<wasmi::FuncRef, wasmi::Error> {
        let func_ref = match field_name {
            "add_ones_wasm" => FuncInstance::alloc_host(
        Signature::new(&[][..], Some(ValueType::I32) ),
                ADD_ONE_WASM_INDEX,
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
            ADD_ONE_WASM_INDEX => {
                Ok(Some(add_ones_wasm().into()))
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
    
    let result_opt = instance.invoke_export::<Runtime>("dds_start", &[], &mut runtime).unwrap();
    
    Ok(())
}
