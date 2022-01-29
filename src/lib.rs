//! # Rust bindings for Orbiter SDK
//!  
//! This crate allows the development of addon modules for the [Orbiter spaceflight simulator](http://orbit.medphys.ucl.ac.uk/) in Rust. At present, this crate is still under development
//! and only a limited subset of the `VESSEL` API is supported.
//!
//! ## Minimal Example
//!
//! The following example shows a very basic addon that uses the existing textures supplied with Orbiter to create a minimal vessel addon.
//!
//! ```
//! use orbiter_rs::{
//!    debug_string, OrbiterVessel, init_vessel, SDKVessel
//! };
//! pub struct MinimalPB{
//!     ctx: SDKVessel
//! }
//! impl MinimalPB {
//!     pub fn new(vessel: SDKVessel) -> Self {
//!         Self {
//!             ctx: vessel
//!         }
//!     }
//! }
//! impl OrbiterVessel for MinimalPB {
//!     fn set_class_caps(&mut self, _cfg: FileHandle) {
//!         self.ctx.SetSize(1.0);
//!         self.ctx.AddMesh("ShuttlePB");
//!     }
//!     fn on_pre_step(&mut self, sim_t: f64, _sim_dt: f64, _mjd: f64)
//!     {
//!         debug_string!("Hello world! sim_t: {:.2}", sim_t);
//!     }
//! }
//!
//! init_vessel!(
//!     fn init(vessel) {
//!         MinimalPB::new(vessel)
//!     }
//!     fn exit() {}
//! );
//! ```
//!
//! The `Cargo.toml` for the addon must specify `crate-type = [cdylib]`. It may also be beneficiant to create a `.cargo/confg` file with the following contents:
//!
//! ```toml
//! [build]
//! target = "i686-pc-windows-msvc"
//! ```
//!
//! This will ensure that the build always targets 32-bit windows (which is the only platform supported by Orbiter at the time of writing).
//! The `ORBITER_DIR` environment variable must be set to the path to a working Orbiter installation before building.
//!
//! ## Building and Installing Examples
//!
//! The examples in the `examples` directory can be built by running `cargo build --example MinimalPB` and `cargo build --example Surveyor`. To install these, copy the resulting DLL from
//! `target/i686-pc-windows-msvc/debug/examples/MinimalPB.dll` to the `Modules` folder in your Orbiter installation. The contents of `examples/MinimalPB/Config` and `examples/MinimalPB/Scenarios` should also be copied
//! into your Orbiter installation direction under the respective folders.
//!
//! For the `Surveyor` example, the `Meshes`, `Textures` folders also needs to have their contents copied to the respective folders under the Orbiter installation
//!
//! The addons can then be tested by opening the newly installed scenarios in Orbiter.
//!
//! ## Logging
//!
//! `orbiter-rs` uses the [log] crate to facilitate logging directly to the Orbiter log. Any addons seeking to use this must call [`init_logging`] somewhere in their code, preferably in their [OrbiterVessel::set_class_caps] implementation.
//! After the system is initialized, the macros [log::error], [log::warn], [log::info], [log::debug] and [log::trace] can be used. All of the [filtering features](log#compile-time-filters) of the log crate may be used as well.
//!

use cxx::ExternType;

mod macros;

mod vessel;
pub use vessel::OrbiterVessel;

mod input;
pub use input::*;

mod logging;
pub use logging::init_logging;
pub use logging::OrbiterLogger;

// FFI interface to orbiter.rs
include!("ffi.rs");
