pub mod oapi_consts;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("src/spacecraft.h");
        type BoxDynVessel = Box<dyn crate::OrbiterVessel>;
        type PtrBoxDynVessel = crate::PtrBoxDynVessel;

        type SpacecraftWrapper;
        type VECTOR3;

        fn SetSize(self: &SpacecraftWrapper, size: f64);
        fn AddMesh(self: &SpacecraftWrapper, mesh_name: &str);
        // fn _V(x: f64, y: f64, z: f64) -> &VECTOR3;
        fn debugLog(s: &str);
    }
    extern "Rust" {
        fn create_rust_spacecraft() -> BoxDynVessel;
        fn dyn_vessel_set_class_caps(vessel: &BoxDynVessel, context: &SpacecraftWrapper);
        fn dyn_vessel_pre_step(vessel: &mut BoxDynVessel, context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64);
        unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel);
    }
}

// Based on https://github.com/dtolnay/cxx/pull/672
use cxx::ExternType;
use ffi::SpacecraftWrapper;
pub trait OrbiterVessel {
    fn set_class_caps(&self, context: &SpacecraftWrapper);
    fn pre_step(&mut self, context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64);
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
fn dyn_vessel_set_class_caps(vessel: &Box<dyn OrbiterVessel>, context: &SpacecraftWrapper,)
{
    (**vessel).set_class_caps(context);
}
fn dyn_vessel_pre_step(vessel: &mut Box<dyn OrbiterVessel>, context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64)
{
    (**vessel).pre_step(context, sim_t, sim_dt, mjd);
}

mod macros;

pub struct RustSpacecraft{}
impl OrbiterVessel for RustSpacecraft {
    fn set_class_caps(&self, context: &SpacecraftWrapper) {
        ffi::debugLog("Hello world!");
        context.SetSize(1.0);
        context.AddMesh("Wheel")
    }
    fn pre_step(&mut self, _context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64)
    {
        ffi::debugLog(& format!("Hello world! {} {} {}", sim_t, sim_dt, mjd));
    }
}

make_orbiter_vessel!(RustSpacecraft{});
