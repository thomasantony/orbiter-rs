## Orbiter spacecraft addon development in Rust

This project is a proof of concept for creating a spacecraft addon for the [Orbiter](https://github.com/orbitersim/orbiter) spaceflight simulator in Rust. It uses the [cxx](https://www.cxx.rs) crate creating Rust bindings for the Orbiter SDK. 

### Goals

The initial goal of this project is to be able to re-create the core logic from this [tutorial](https://www.orbiterwiki.org/wiki/Vessel_Tutorial_1) in Rust. The source-code from the tutorial can be found in [src/cpp/Surveyor.cpp](src/cpp/Surveyor.cpp). This file exists purely as a reference and is not used during the build. Only the functions/classes required for this tutorial is currently implemented for Rust. This has been completed and it is possible to build a DLL file that can be loaded in Orbiter and re-creates the functionality of the Surveyor spacecraft to the extent shown in the aforementioned tutorial.

### Implementation
As of now, all the spacecraft-specific implementation details can be found in [`src/surveyor.rs`](src/surveyor.rs). An addon module must include a struct implementing the `OrbiterVessel` trait and it must call the `make_orbiter_vessel!` macro or provide its own `create_rust_spacecraft` function. In order to link this code to the final DLL, [`src/lib.rs`](src/lib.rs) has the following stub:

```rust
mod surveyor;
pub use surveyor::create_rust_spacecraft;
```

### Building

This addon has been tested with Rust 1.57.0 and Visual Studio 2019 Commuity Edition on Windows 10. Running `cargo build` should build the project generate a DLL file.

### Installing and Testing the Addon

Once you build the addon, you should have a file called `orbiter_rs.dll` in `target\i686-pc-windows-msvc\Debug`. Copy this file to the `Modules` folder in your Orbiter installation and rename it to `Surveyor.dll`. Also copy over the files in the `Config`, `Meshes` and `Scenarios` folders into the corresponding folders in your Orbiter installation. Launch the `SurveyorInOrbit` scenario in Orbiter and make sure that the spacecraft shows up. Pressing "L" should activate the retro thruster firing sequence.

### Future

Right now, the only way to build a Rust addon is to clone/fork this repository and modify the spacecraft specific code. Due to limitations of the `cxx` library and my own lack of knowledge about it currently prevents me from making this into a crate that can be freely pulled into addon-projects. Do this will probably require custom code generation using the `gen` module from `cxx`. I intend to tackle this at some point.

### Meshes

The meshes in the demo addon were adapted from the [Surveyor 1.0 Orbiter Addon](https://www.orbithangar.com/showAddon.php?id=e69853be-2dd6-4b37-a5df-fe6827c01cae). These were updated based on the tutorial at [https://www.orbiterwiki.org/wiki/Vessel_Tutorial_1](https://www.orbiterwiki.org/wiki/Vessel_Tutorial_1).

### MFDs

An older proof-of-concept for building MFDs can be found at [https://github.com/thomasantony/RustMFD](https://github.com/thomasantony/RustMFD). However, this uses an outdated and modified version of `cxx` and may not build at this time and exists just as a reference. The code in that repo will be moved over to this one at some point in the future.

### Notes
- Uses `.cargo/config` to force i686 as target

