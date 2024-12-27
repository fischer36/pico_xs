use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    //cc::Build::new()
    // eI
    //    .include("src/include")
    //    .file("src/vector_table/reset_handler.c")
    //    .compile("ffi");
    let out21 = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    File::create("./helloxd.txt")
        .unwrap()
        .write_all(b"XD")
        .unwrap();
    println!("HAHHAH XD {:?}", out21);
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("boot2.bin"))
        .unwrap()
        .write_all(include_bytes!("boot2.bin"))
        .unwrap();

    let link = include_bytes!("link.ld");
    let mut link_file = File::create(out.join("link.ld")).unwrap();
    link_file.write_all(link).unwrap();
    println!("cargo:rustc-link-arg=-Tlink.ld");
    println!("cargo:rustc-link-arg=--nmagic");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.ld");
}
