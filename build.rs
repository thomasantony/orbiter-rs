fn main() {
    cxx_build::bridge("src/ffi.rs")
        .file("src/cpp/vessel_context.cpp")
        .include(".")
        .include("./include")
        .include("D:\\code\\Orbitersdk\\include\\")
        .flag_if_supported("-std=c++14")
        .compile("orbiter-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/spacecraft.h");
    println!("cargo:rerun-if-changed=src/spacecraft.cpp");
    println!("cargo:rustc-link-lib=Orbiter");
    println!("cargo:rustc-link-lib=Orbitersdk");
    println!("cargo:rustc-link-search=D:\\code\\Orbitersdk\\lib");
}
