orbiter-rs &mdash; Rust Bindings for the Orbiter SDK
================================================

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/orbiter-rs.svg
[crates-url]: https://crates.io/crates/orbiter-rs
[docs-badge]: https://img.shields.io/badge/docs.rs-rustdoc-green
[docs-url]: https://docs.rs/orbiter-rs/
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/thomasantony/orbiter-rs/blob/master/LICENSE
<!-- [actions-badge]: https://github.com/tokio-rs/tokio/workflows/CI/badge.svg
[actions-url]: https://github.com/tokio-rs/tokio/actions?query=workflow%3ACI+branch%3Amaster
-->

This library was created to develop addons for the [Orbiter](https://github.com/orbitersim/orbiter) spaceflight simulator in Rust. It uses the [cxx](https://www.cxx.rs) crate for creating Rust bindings for the Orbiter SDK. A fairly complicated addon complete with a landing guidance system built using these bindings can be found at: [https://github.com/thomasantony/surveyor](https://github.com/thomasantony/surveyor)

### Implementation

The crate can now be imported and used like a library. A demo implementation can be found in [`examples/Surveyor/surveyor.rs`](examples/Surveyor/surveyor.rs). An addon module must include a struct implementing the `OrbiterVessel` trait. Then it must use the `init_vessel!` macro to generate the code that links it to Orbiter SDK. For example:

```rust
init_vessel!(
    fn init(vessel) {
        Surveyor::new(vessel)
    }
    fn exit() {}
);
```

### Lifecycle of an Orbiter Rust Addon

TODO

### Building

1. Download/Install Orbiter
2. Install Visual Studio 2019
3. Install Rust using `rustup` (https://rustup.rs)
4. Install the win32 target by running `rustup target add i686-pc-windows-msvc`

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
