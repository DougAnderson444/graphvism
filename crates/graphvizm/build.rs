//! Build script to generate Wasmtime component bindings for the `viz` world.
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // 1. Copy wasm file to OUT_DIR for the include_bytes! macro
    let wasm_src_path = "vendor/viz.wasm";
    fs::copy(wasm_src_path, out_dir.join("viz.wasm")).expect("Failed to copy viz.wasm");
    println!("cargo:rerun-if-changed={}", wasm_src_path);

    // 2. Read the WIT source file
    let wit_path = "vendor/world.wit";
    let wit_src = fs::read_to_string(wit_path).expect("Failed to read world.wit");
    println!("cargo:rerun-if-changed={}", wit_path);

    // 3. Generate a bindings.rs file in OUT_DIR.
    // First, create the raw string literal containing the WIT source.
    let inline_wit = format!("r##\"{}\"##", wit_src);

    // Then, embed that string literal into the bindgen! macro call.
    let bindings_code = format!(
        r#"
        wasmtime::component::bindgen!({{
            world: "viz",
            inline: {},
        }});
        "#,
        inline_wit
    );

    let dest_path = out_dir.join("bindings.rs");
    fs::write(&dest_path, bindings_code).expect("Failed to write bindings file");
}

