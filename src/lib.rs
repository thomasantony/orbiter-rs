pub mod oapi_consts;
pub mod utils;



// // ctype_wrapper!(THRUSTERHANDLE, &std::os::raw::c_void, "THRUSTERHANDLE");
// #[derive(Debug, Eq, Clone, PartialEq, Hash)]
// #[allow(non_camel_case_types)]
// #[repr(transparent)]
// pub struct THRUSTERHANDLE(pub *const std::ffi::c_void);
// unsafe impl cxx::ExternType for THRUSTERHANDLE {
//     type Id = cxx::type_id!("std::ffi::c_void");
//     type Kind = cxx::kind::Trivial;
// }
// #[derive(Debug, Eq, Clone, PartialEq, Hash)]
// #[allow(non_camel_case_types)]
// #[repr(transparent)]
// pub struct PROPELLANTHANDLE(pub *const std::ffi::c_void);
// unsafe impl cxx::ExternType for PROPELLANTHANDLE {
//     type Id = cxx::type_id!("std::ffi::c_void");
//     type Kind = cxx::kind::Trivial;
// }
// // ctype_wrapper!(PROPELLANTHANDLE, &std::os::raw::c_void, "PROPELLANTHANDLE");

#[cxx::bridge]
pub mod ffi {
    pub struct Vector3
    {
        x: f64,
        y: f64,
        z: f64
    }

    unsafe extern "C++" {
        include!("src/spacecraft.h");
        type c_void;

        type BoxDynVessel = Box<dyn crate::OrbiterVessel>;
        type PtrBoxDynVessel = crate::PtrBoxDynVessel;

        type SpacecraftWrapper;
        type VECTOR3;

        // VESSEL API
        fn SetSize(self: &SpacecraftWrapper, size: f64);
        fn AddMesh(self: &SpacecraftWrapper, mesh_name: &str);
        fn SetPMI(self: &SpacecraftWrapper, pmi: &Vector3);
        fn CreatePropellantResource(self: &SpacecraftWrapper, mass: f64) -> usize;
        fn CreateThruster(self: &SpacecraftWrapper, pos: &Vector3, dir: &Vector3, maxth0: f64, ph: usize, isp: f64) -> usize;
        fn CreateThrusterGroup(self: &SpacecraftWrapper, thrusters: &Vec<usize>) -> usize;
/// SetTouchdownPoints
/// CreateThrusterGroup
/// AddExhaust
/// SetCameraOffset
/// SetEmptyMass
/// GetThrusterGroupLevel
/// SetThrusterDir
/// GetPropellantMass
/// SetThrusterLevel
/// oapiCreateVessel
/// Local2Rel
/// GetStatus
/// SpawnObject
        fn debugLog(s: &str);
    }
    extern "Rust" {
        fn create_rust_spacecraft() -> BoxDynVessel;
        fn dyn_vessel_set_class_caps(vessel: &BoxDynVessel, context: &SpacecraftWrapper);
        fn dyn_vessel_pre_step(vessel: &mut BoxDynVessel, context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64);
        unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel);
    }
}

use ffi::Vector3;
impl Vector3
{
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z}
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
        context.SetSize(1.0);
        context.SetPMI(_V!(0.50, 0.50, 0.50));

        const VERNIER_PROP_MASS:f64 = 70.98;
        let ph_vernier = context.CreatePropellantResource (VERNIER_PROP_MASS);
        let th_vernier = context.CreateThruster(_V!(0.0, 1.0, 1.0), _V!(0.0, 0.0, 1.0), 10.0, ph_vernier, 10.0);
        context.CreateThrusterGroup(&vec![th_vernier]);
        context.AddMesh("ShuttlePB");
    }
    fn pre_step(&mut self, _context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64)
    {
        ffi::debugLog(& format!("Hello world! {} {} {}", sim_t, sim_dt, mjd));
    }
}

make_orbiter_vessel!(RustSpacecraft{});
