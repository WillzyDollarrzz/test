// build.rs

// This build script reads your Anchor IDL JSON and generates
// a Rust CPI client into src/pump_fun.rs on each build.

use std::{fs, process::exit};
use anchor_idl::GeneratorOptions;

fn main() {
    // 1) Point at the IDL file:
    let opts = GeneratorOptions {
        idl_path: "pump_fun.json".to_string(),
        ..Default::default()
    };

    // 2) Create the generator:
    let generator = opts.to_generator();

    // 3) Generate the CPI interface directly (returns TokenStream):
    let tokens = generator.generate_cpi_interface();

    // 4) Write out the generated code to src/pump_fun.rs
    if let Err(e) = fs::write("src/pump_fun.rs", tokens.to_string()) {
        eprintln!("Unable to write src/pump_fun.rs: {}", e);
        exit(1);
    }

    // Re-run build script when the IDL changes
    println!("cargo:rerun-if-changed=pump_fun.json");
}
