use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .include("src/include")
        .file("src/vector_table/reset_handler.c")
        .compile("ffi");

    // Put `boot2.bin` in our output directory and ensure it's on the linker search path.
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("boot2.bin"))
        .unwrap()
        .write_all(include_bytes!("boot2.bin"))
        .unwrap();

    println!("cargo:rustc-link-lib=static=ffi");
    println!("cargo:rustc-Cllvm-args=--inline-threshold=5"); // Set inline threshold
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=--nmagic");

    // Include `link.ld` at compile time, create it in the output directory, and write contents
    let link = include_bytes!("link.ld");
    let mut link_file = File::create(out.join("link.ld")).unwrap();
    link_file.write_all(link).unwrap();
    let linker_script_path = out.join("link.ld");
    println!("cargo:rustc-link-arg=-T{}", linker_script_path.display());
}
