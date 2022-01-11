// The OrbiterSDK detection code of this build script is from the emgre/orbiter-rs repository
use std::env;
use std::path::PathBuf;

macro_rules! error {
    ($($args:tt)+) => ({
        let msg = format!($($args)*);
        println!("cargo:warning={}", msg);
        panic!("{}", msg);
    })
}

fn main() {
    const ORBITER_DIR_ENV: &str = "ORBITER_DIR";
    const ORBITER_SDK_ENV: &str = "ORBITER_SDK";

    // Check target triple for MSVC 32-bit
    if env::var("TARGET").unwrap() != "i686-pc-windows-msvc" {
        error!("Orbiter plugins must use the `i686-pc-windows-msvc` target");
    }

    // Extract OrbiterSDK location
    let orbiter_sdk_path = if let Ok(sdk_path) = env::var(ORBITER_SDK_ENV) {
        PathBuf::from(sdk_path)
    } else if let Ok(orbiter_path) = env::var(ORBITER_DIR_ENV) {
        [&orbiter_path, "Orbitersdk"].iter().collect::<PathBuf>()
    } else {
        error!(
            "{} or {} environment must be set",
            ORBITER_DIR_ENV, ORBITER_SDK_ENV
        );
    };
    let orbiter_lib_path = orbiter_sdk_path.join("lib");
    let orbiter_include_path = orbiter_sdk_path.join("include");

    // Check OrbiterSDK installation
    if !orbiter_lib_path.join("orbiter.lib").is_file() {
        error!(
            "{} does not contain orbiter.lib",
            orbiter_lib_path.to_string_lossy()
        );
    }
    if !orbiter_lib_path.join("Orbitersdk.lib").is_file() {
        error!(
            "{} does not contain Orbitersdk.lib",
            orbiter_lib_path.to_string_lossy()
        );
    }
    if !orbiter_include_path.join("Orbitersdk.h").is_file() {
        error!(
            "{} does not contain Orbitersdk.h",
            orbiter_include_path.to_string_lossy()
        );
    }
    cxx_build::bridge("src/ffi.rs")
        .file("src/cpp/vessel_context.cpp")
        .file("src/cpp/box_dyn_vessel.cpp")
        .include(".")
        .include("./include")
        .include(orbiter_include_path)
        .flag_if_supported("-std=c++14")
        .compile("orbiter-rs");

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=include/box_dyn_vessel.h");
    println!("cargo:rerun-if-changed=include/vessel_context.h");
    println!("cargo:rerun-if-changed=src/cpp/vessel_context.cpp");
    println!("cargo:rustc-link-lib=Orbiter");
    println!("cargo:rustc-link-lib=Orbitersdk");
    println!("cargo:rustc-link-search={}", orbiter_lib_path.to_string_lossy());
}
