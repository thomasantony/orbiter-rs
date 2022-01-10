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
        fn AddExhaust (self: &SpacecraftWrapper, th: usize, lscale: f64, wscale:f64) -> usize;
        fn SetPMI(self: &SpacecraftWrapper, pmi: &Vector3);
        fn CreatePropellantResource(self: &SpacecraftWrapper, mass: f64) -> usize;
        fn CreateThruster(self: &SpacecraftWrapper, pos: &Vector3, dir: &Vector3, maxth0: f64, ph: usize, isp: f64) -> usize;
        fn CreateThrusterGroup(self: &SpacecraftWrapper, thrusters: &[usize]) -> usize;
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
        fn dyn_vessel_set_class_caps(vessel: &mut BoxDynVessel, context: &SpacecraftWrapper);
        fn dyn_vessel_pre_step(vessel: &mut BoxDynVessel, context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64);
        unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel);
    }
}

pub use ffi::Vector3;
pub use ffi::debugLog;

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
    fn set_class_caps(&mut self, context: &SpacecraftWrapper);
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
fn dyn_vessel_set_class_caps(vessel: &mut Box<dyn OrbiterVessel>, context: &SpacecraftWrapper,)
{
    (**vessel).set_class_caps(context);
}
fn dyn_vessel_pre_step(vessel: &mut Box<dyn OrbiterVessel>, context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64)
{
    (**vessel).pre_step(context, sim_t, sim_dt, mjd);
}

mod macros;


const VERNIER_PROP_MASS:f64 = 70.98;
const VERNIER_ISP:f64 = 3200.0;
const VERNIER_THRUST:f64 = 463.0;
const VERNIER_RAD:f64 = 0.86;
const VERNIER_Z:f64 = -0.5;

const RCS_PROP_MASS:f64 = 2.0;
const RCS_ISP:f64 = 630.0;
const RCS_THRUST:f64 = 0.25;
const RCS_RAD:f64 = 1.0;
const RCS_STA:f64 = -0.5;
const RCS_SPACE:f64 = 0.1;

const RETRO_PROP_MASS:f64 = 560.64;
const RETRO_THRUST:f64 = 39140.0;
const RETRO_BURNTIME:f64 = 40.5;
const RETRO_ITOT:f64 = RETRO_THRUST * RETRO_BURNTIME;
const RETRO_ISP:f64 = RETRO_ITOT / RETRO_PROP_MASS;
const RETRO_STA:f64 = -0.75;

const LANDER_EMPTY_MASS:f64 = 289.10; //Basic bus plus payload minus AMR minus retro case
const RETRO_EMPTY_MASS:f64 = 64.88;
const AMR_MASS:f64 = 3.82;

const LEG_RAD:f64 = 1.5;
const LEG_STA:f64 = -0.6;

pub struct RustSpacecraft
{
    th_vernier: Vec<usize>,
}
impl RustSpacecraft {
    pub fn new() -> Self {
        Self {
            th_vernier: Vec::new()
        }
    }
}
impl OrbiterVessel for RustSpacecraft {
    fn set_class_caps(&mut self, context: &SpacecraftWrapper) {
        context.SetSize(1.0);
        context.SetPMI(_V!(0.50, 0.50, 0.50));

        let ph_vernier = context.CreatePropellantResource (VERNIER_PROP_MASS);
        self.th_vernier.push(context.CreateThruster(_V!(                          0.0 * VERNIER_RAD,  1.0 * VERNIER_RAD, VERNIER_Z), _V!(0.0, 0.0, 1.0), VERNIER_THRUST, ph_vernier, VERNIER_ISP));
        self.th_vernier.push(context.CreateThruster(_V!(( 60.0f64).to_radians().sin() * VERNIER_RAD, -0.5 * VERNIER_RAD, VERNIER_Z), _V!(0.0, 0.0, 1.0), VERNIER_THRUST, ph_vernier, VERNIER_ISP));
        self.th_vernier.push(context.CreateThruster(_V!((-60.0f64).to_radians().sin() * VERNIER_RAD, -0.5 * VERNIER_RAD, VERNIER_Z), _V!(0.0, 0.0, 1.0), VERNIER_THRUST, ph_vernier, VERNIER_ISP));
        context.CreateThrusterGroup(&self.th_vernier);
        for th in self.th_vernier.iter() {
            context.AddExhaust(*th, 1.0, 0.1);
        }

        context.AddMesh("ShuttlePB");
    }
    fn pre_step(&mut self, _context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64)
    {
        // ffi::debugLog(& format!("{:?}", self.th_vernier));
        debugLog(& format!("Hello world! {} {} {}", sim_t, sim_dt, mjd));
    }
}

make_orbiter_vessel!(RustSpacecraft::new());
