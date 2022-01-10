pub mod oapi_consts;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("src/spacecraft.h");
        type BoxDynVessel = Box<dyn crate::OrbiterVessel>;
        type PtrBoxDynVessel = crate::PtrBoxDynVessel;

        fn debugLog(s: &str);
    }
    extern "Rust" {
        type RustSpacecraft;
        
        fn create_rust_spacecraft() -> BoxDynVessel;
        fn dyn_vessel_set_class_caps(vessel: &BoxDynVessel);
        fn dyn_vessel_pre_step(vessel: &mut BoxDynVessel, sim_t: f64, sim_dt: f64, mjd: f64);
        unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel);
    }
}

// Based on https://github.com/dtolnay/cxx/pull/672
use cxx::ExternType;
pub trait OrbiterVessel {
    fn set_class_caps(&self);
    fn pre_step(&mut self, sim_t: f64, sim_dt: f64, mjd: f64);
}
unsafe impl ExternType for Box<dyn OrbiterVessel> {
    type Id = cxx::type_id!("BoxDynVessel");
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
pub struct PtrBoxDynVessel(*mut Box<dyn OrbiterVessel>);
unsafe impl ExternType for PtrBoxDynVessel {
    type Id = cxx::type_id!("PtrBoxDynVessel");
    type Kind = cxx::kind::Trivial;
}
unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel) {
    std::ptr::drop_in_place(ptr.0);
}

// trait fn shims
fn dyn_vessel_set_class_caps(vessel: &Box<dyn OrbiterVessel>)
{
    (**vessel).set_class_caps();
}
fn dyn_vessel_pre_step(vessel: &mut Box<dyn OrbiterVessel>, sim_t: f64, sim_dt: f64, mjd: f64)
{
    (**vessel).pre_step(sim_t, sim_dt, mjd);
}




pub struct RustSpacecraft;
impl OrbiterVessel for RustSpacecraft {
    fn set_class_caps(&self) {
        ffi::debugLog("Hello world!");
    }
    fn pre_step(&mut self, sim_t: f64, sim_dt: f64, mjd: f64)
    {
        ffi::debugLog(& format!("Hello world! {} {} {}", sim_t, sim_dt, mjd));
    }
}

#[no_mangle]
fn create_rust_spacecraft() -> Box<dyn OrbiterVessel> {
    Box::new(RustSpacecraft)
}
