use cxx::ExternType;

pub mod consts;
mod macros;

mod vessel;
pub use vessel::OrbiterVessel;

// FFI interface to orbiter.rs
include!("ffi.rs");

/// Spacecraft implementation
mod surveyor;
pub use surveyor::create_rust_spacecraft;
