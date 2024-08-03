use std::env;
use std::fs;
use std::path::{Path, PathBuf};
fn main() {
    // println!("cargo:rustc-env=TARGET=thumbv6m-none-eabi");
    cc::Build::new()
        .file("src/vector_table/reset_handler.c")
        .file("src/asm/asm.s")
        .compile("ffi"); // Compiles and links the C code into a static library

    // Handling memory.x
    let out_dir = env::var("OUT_DIR").unwrap();
    let memory_x_path = PathBuf::from(&out_dir).join("memory.x");
    fs::copy("memory.x", &memory_x_path).unwrap();
    println!(
        "cargo:rustc-link-search={}",
        memory_x_path.parent().unwrap().display() // Ensure the directory is added to the linker search path
    );

    // Handling link.ld
    let link_ld_path = PathBuf::from(&out_dir).join("link.ld");
    fs::copy("link.ld", &link_ld_path).unwrap();
    println!("cargo:rustc-link-arg=-T{}", link_ld_path.display()); // Pass the linker script to the linker

    // Specify files to watch for changes
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rerun-if-changed=src/vector_table/reset_handler.c");
}
