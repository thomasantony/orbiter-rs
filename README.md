## Orbiter spacecraft addon development in Rust

This project is a proof of concept for creating a spacecraft addon for the [Orbiter](https://github.com/orbitersim/orbiter) spaceflight simulator in Rust. It uses the [cxx](https://www.cxx.rs) crate creating Rust bindings for the Orbiter SDK. 

### Goals

The initial goal of this project is to be able to re-create the core logic from this [tutorial](https://www.orbiterwiki.org/wiki/Vessel_Tutorial_1) in Rust. The source-code from the tutorial can be found in [examples/Surveyor.cpp](examples/Surveyor.cpp). This file exists purely as a reference and is not used during the build. Only the functions/classes required for this tutorial is currently implemented for Rust. This has been completed and it is possible to build a DLL file that can be loaded in Orbiter and re-creates the functionality of the Surveyor spacecraft to the extent shown in the aforementioned tutorial.

### Implementation

The crate can now be imported and used like a library. A demo implementation can be found in [`src/examples/surveyor.rs`](src/examples/surveyor.rs). An addon module must include a struct implementing the `OrbiterVessel` trait. Then it must use the `init_vessel!` macro to generate the code that links it to Orbiter SDK. For example:

```rust
init_vessel!(
    fn init(_h_vessel: OBJHANDLE, _flight_model: i32) {
        Surveyor::default()
    }
    fn exit() {}
);
```

### Building

1. Download/Install Orbiter
2. Install Visual Studio 2019
3. Install Rust using `rustup` (https://rustup.rs)
4. Install the win32 target by running `rustup add target i686-pc-windows-msvc`

This addon has been tested with Rust 1.57.0 and Visual Studio 2019 Commuity Edition on Windows 10. Running `cargo build` should build the project generate a DLL file.

### Installing and Testing the Addon

Once you build the addon, you should have a file called `Surveyor.dll` in ` target/i686-pc-windows-msvc/debug/examples/`. Copy this file to the `Modules` folder in your Orbiter installation. Also copy over the files in the `Config`, `Meshes` and `Scenarios` folders into the corresponding folders in your Orbiter installation. Launch the `SurveyorInOrbit` scenario in Orbiter and make sure that the spacecraft shows up. Pressing "L" should activate the retro thruster firing sequence.

### Future

Only a limited number of Orbiter functions are now available to Rust bindings. This list will expand in the future to hopefully include all of Orbiter SDK. Pull requests are welcome!

### Meshes

The meshes in the demo addon were adapted from the [Surveyor 1.0 Orbiter Addon](https://www.orbithangar.com/showAddon.php?id=e69853be-2dd6-4b37-a5df-fe6827c01cae). These were updated based on the tutorial at [https://www.orbiterwiki.org/wiki/Vessel_Tutorial_1](https://www.orbiterwiki.org/wiki/Vessel_Tutorial_1).

### MFDs

An older proof-of-concept for building MFDs can be found at [https://github.com/thomasantony/RustMFD](https://github.com/thomasantony/RustMFD). However, this uses an outdated and modified version of `cxx` and may not build at this time and exists just as a reference. The code in that repo will be moved over to this one at some point in the future.

### Notes
- Uses `.cargo/config` to force i686 as target
