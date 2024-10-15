use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    //cc::Build::new()
    //    .include("src/include")
    //    .file("src/vector_table/reset_handler.c")
    //    .compile("ffi");

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // Include `link.ld` at compile time, create it in the output directory, and write contents
    // Put `boot2.bin` in our output directory and ensure it's on the linker search path.
    File::create(out.join("boot2.bin"))
        .unwrap()
        .write_all(include_bytes!("boot2.bin"))
        .unwrap();

    // println!("cargo:rustc-link-lib=static=ffi");
    // println!("cargo:rustc-Cllvm-args=--inline-threshold=5"); // Set inline threshold
    let link = include_bytes!("link.ld");
    let mut link_file = File::create(out.join("link.ld")).unwrap();
    link_file.write_all(link).unwrap();
    println!("cargo:rustc-link-arg=-Tlink.ld");
    println!("cargo:rustc-link-arg=--nmagic");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.ld");
}
