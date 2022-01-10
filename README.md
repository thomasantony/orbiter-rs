## Orbiter spacecraft addon in Rust

This project is a proof of concept for creating a spacecraft addon for the [Orbiter](https://github.com/orbitersim/orbiter) spaceflight simulator in Rust. It uses the [cxx](https://www.cxx.rs) crate for building wrapper to the C++ code in the Orbiter SDK. 


### Goals

The initial goal of this project is to be able to re-create the core logic from [tutorial](https://www.orbiterwiki.org/wiki/Vessel_Tutorial_1) in Rust. Only the function required to implement this is currently accessible from Rust. 


### MFDs

An older proof-of-concept for building MFDs can be found at [https://github.com/thomasantony/RustMFD](https://github.com/thomasantony/RustMFD). However, this uses an outdated and modified version of `cxx` and may not build at this time and exists just as a reference.

### Notes
- Use `.cargo/config` to force i686 as target
- Copy the RustySpace.cfg (or similar) config file to ensure that the spacecraft is detected by Orbiter

