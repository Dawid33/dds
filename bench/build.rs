//use std::{path::{Path, PathBuf}, env};

fn main() {
    // println!("cargo:rerun-if-changed=src/hello.c");

    // let output_path = get_output_path();
    // let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("test.html");
    // let output_path = Path::new(&output_path).join("test.html");
    // std::fs::copy(input_path, output_path).unwrap();
}

// fn get_output_path() -> PathBuf {
//     //<root or manifest path>/target/<profile>/
//     let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
//     let build_type = env::var("PROFILE").unwrap();
//     let path = Path::new(&manifest_dir_string).parent().unwrap().join("target").join(build_type);
//     return PathBuf::from(path);
// }
