use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let linker = include_bytes!("link.ld");
    let mut f = File::create(out.join("link.ld")).unwrap();
    f.write_all(linker).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=-Tlink.ld");
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rerun-if-changed=build.rs");
}
