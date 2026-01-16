use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = PathBuf::from(&crate_dir).join("include");

    std::fs::create_dir_all(&output_dir).expect("Failed to create include directory");

    // Generate libelf.h using cbindgen (enums, opaque types, function declarations)
    // elf.h and gelf.h are maintained manually since they're just constants/macros/typedefs
    /*
    let config_path = PathBuf::from(&crate_dir).join("cbindgen.toml");
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(cbindgen::Config::from_file(config_path).expect("Unable to load cbindgen.toml"))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(output_dir.join("libelf.h"));
    */

    // Provide metadata to dependent crates via DEP_ELF_* environment variables
    // The `links` key in Cargo.toml enables this
    println!("cargo:include={}", output_dir.display());

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=include/");
}
