pub mod oapi_consts;
pub mod utils;

#[allow(dead_code)]
pub struct VECTOR3([f64; 3]);
impl VECTOR3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }
}
unsafe impl cxx::ExternType for VECTOR3 {
    type Id = cxx::type_id!("VECTOR3");
    type Kind = cxx::kind::Trivial;
}
use VECTOR3 as Vector3;

ctype_wrapper!(THRUSTER_HANDLE, usize);
ctype_wrapper!(PROPELLANT_HANDLE, usize);
type ThrusterHandle = THRUSTER_HANDLE;
type PropellantHandle = PROPELLANT_HANDLE;

#[cxx::bridge]
pub mod ffi {
    
    #[derive(Debug)]
    #[repr(usize)]
    // #[cxx_name = "THGROUP_TYPE"]
    enum THGROUP_TYPE {
        #[cxx_name = "THGROUP_MAIN"]
        Main,
        #[cxx_name = "THGROUP_RETRO"]
        Retro,
        #[cxx_name = "THGROUP_HOVER"]
        Hover,
        #[cxx_name = "THGROUP_ATT_PITCHUP"]
        AttPitchup,
        #[cxx_name = "THGROUP_ATT_PITCHDOWN"]
        AttPitchdown,
        #[cxx_name = "THGROUP_ATT_YAWLEFT"]
        AttYawleft,
        #[cxx_name = "THGROUP_ATT_YAWRIGHT"]
        AttYawright,
        #[cxx_name = "THGROUP_ATT_BANKLEFT"]
        AttBankleft,
        #[cxx_name = "THGROUP_ATT_BANKRIGHT"]
        AttBankright,
        #[cxx_name = "THGROUP_ATT_RIGHT"]
        AttRight,
        #[cxx_name = "THGROUP_ATT_LEFT"]
        AttLeft,
        #[cxx_name = "THGROUP_ATT_UP"]
        AttUp,
        #[cxx_name = "THGROUP_ATT_DOWN"]
        AttDown,
        #[cxx_name = "THGROUP_ATT_FORWARD"]
        AttForward,
        #[cxx_name = "THGROUP_ATT_BACK"]
        AttBack,
        #[cxx_name = "THGROUP_USER"]
        User = 0x40,
    }
    enum SmallPrime {
        Two = 2,
        Three = 3,
        Five = 5,
        Seven = 7,
    }
    unsafe extern "C++" {
        include!("src/spacecraft.h");

        type BoxDynVessel = Box<dyn crate::OrbiterVessel>;
        type PtrBoxDynVessel = crate::PtrBoxDynVessel;

        type VesselContext;
        type VECTOR3 = crate::VECTOR3;
        type PROPELLANT_HANDLE = crate::PropellantHandle;
        type THRUSTER_HANDLE = crate::ThrusterHandle;
        type THGROUP_TYPE;

        // VESSEL API
        fn SetSize(self: &VesselContext, size: f64);
        fn SetPMI(self: &VesselContext, pmi: &VECTOR3);
        fn SetEmptyMass(self: &VesselContext, empty_mass: f64);
        fn SetCameraOffset(self: &VesselContext, camera_offset: &VECTOR3);
        fn SetTouchdownPoints(
            self: &VesselContext,
            pt1: &VECTOR3,
            pt2: &VECTOR3,
            pt3: &VECTOR3,
        );
        fn SetThrusterDir(self: &VesselContext, th: THRUSTER_HANDLE, dir: &VECTOR3);
        fn SetThrusterLevel(self: &VesselContext, th: THRUSTER_HANDLE, level: f64);

        fn AddMesh(self: &VesselContext, mesh_name: &str);
        fn AddMeshWithOffset(self: &VesselContext, mesh_name: &str, ofs: &VECTOR3);
        fn AddExhaust(self: &VesselContext, th: THRUSTER_HANDLE, lscale: f64, wscale: f64) -> usize;

        fn CreatePropellantResource(self: &VesselContext, mass: f64) -> PROPELLANT_HANDLE;
        fn CreateThruster(
            self: &VesselContext,
            pos: &VECTOR3,
            dir: &VECTOR3,
            maxth0: f64,
            ph: PROPELLANT_HANDLE,
            isp: f64,
        ) -> THRUSTER_HANDLE;
        fn CreateThrusterGroup(
            self: &VesselContext,
            thrusters: &[THRUSTER_HANDLE],
            thgroup_type: THGROUP_TYPE,
        ) -> usize;

        fn ClearMeshes(self: &VesselContext);

        fn GetName(self: &VesselContext) -> &str;
        fn GetPropellantMass(self: &VesselContext, ph: PROPELLANT_HANDLE) -> f64;
        fn GetThrusterGroupLevelByType(self: &VesselContext, thgroup_type: THGROUP_TYPE) -> f64;
        /// oapiCreateVessel
        /// Local2Rel
        /// GetStatus
        fn debugLog(s: &str);
    }
    extern "Rust" {
        fn create_rust_spacecraft() -> BoxDynVessel;
        fn dyn_vessel_set_class_caps(vessel: &mut BoxDynVessel, context: &VesselContext);
        fn dyn_vessel_pre_step(
            vessel: &mut BoxDynVessel,
            context: &VesselContext,
            sim_t: f64,
            sim_dt: f64,
            mjd: f64,
        );
        unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel);
    }
}

pub use ffi::*;

// Based on https://github.com/dtolnay/cxx/pull/672
use cxx::ExternType;
pub trait OrbiterVessel {
    fn set_class_caps(&mut self, context: &VesselContext);
    fn pre_step(&mut self, context: &VesselContext, sim_t: f64, sim_dt: f64, mjd: f64);
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
fn dyn_vessel_set_class_caps(vessel: &mut Box<dyn OrbiterVessel>, context: &VesselContext) {
    (**vessel).set_class_caps(context);
}
fn dyn_vessel_pre_step(
    vessel: &mut Box<dyn OrbiterVessel>,
    context: &VesselContext,
    sim_t: f64,
    sim_dt: f64,
    mjd: f64,
) {
    (**vessel).pre_step(context, sim_t, sim_dt, mjd);
}

mod macros;

#[allow(dead_code)]
pub mod surveyor;
pub use surveyor::create_rust_spacecraft;
