fn main() {
    cc::Build::new()
        .file("src/c/reset.c") // Path to your C file
        .file("src/asm/asm.s") // Path
        .include("/home/thinkerpad/pi/pico/pico-sdk/src/c")
        .compile("usb_init"); // Compiles and links the C code into a static library
    println!("cargo:rerun-if-changed=src/usb_init.c");
    println!("cargo:rerun-if-changed=device.x");
}
