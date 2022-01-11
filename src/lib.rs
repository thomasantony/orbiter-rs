use cxx::ExternType;

mod macros;

mod vessel;
pub use vessel::OrbiterVessel;

mod input;
pub use input::*;

// FFI interface to orbiter.rs
include!("ffi.rs");
