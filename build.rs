fn main() {
    cc::Build::new()
        .file("src/vector_table/reset_handler.c") // Path to your C file
        .file("src/asm/asm.s") // Path
        //.file("src/registers/uart.c") // Path
        //.include("src/include/uart.h")
        .compile("ffi"); // Compiles and links the C code into a static library
    println!("cargo:rerun-if-changed=src/vector_table/reset_handler.c");
}
