use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = PathBuf::from(&crate_dir).join("include");

    std::fs::create_dir_all(&output_dir).expect("Failed to create include directory");

    // Headers are maintained manually in include/ directory
    // cbindgen generation is disabled since we need custom types and function declarations
    // that cbindgen doesn't generate correctly for FFI use.

    // cbindgen::Builder::new()
    //     .with_crate(crate_dir)
    //     .with_language(cbindgen::Language::C)
    //     .with_include_guard("LIBELF_H")
    //     .with_sys_include("stdint.h")
    //     .with_sys_include("stddef.h")
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file(output_dir.join("libelf.h"));

    // Provide metadata to dependent crates via DEP_ELF_* environment variables
    // The `links` key in Cargo.toml enables this
    println!("cargo:include={}", output_dir.display());

    println!("cargo:rerun-if-changed=src/");
}
