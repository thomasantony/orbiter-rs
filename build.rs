fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/spacecraft.cpp")
        .include(".")
        .include("D:\\code\\Orbitersdk\\include\\")
        .flag_if_supported("-std=c++14")
        .compile("rust-orbiter-bridge");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/spacecraft.h");
    println!("cargo:rerun-if-changed=src/spacecraft.cpp");
    println!("cargo:rustc-link-lib=Orbiter");
    println!("cargo:rustc-link-lib=Orbitersdk");
    println!("cargo:rustc-link-search=D:\\code\\Orbitersdk\\lib");
}
