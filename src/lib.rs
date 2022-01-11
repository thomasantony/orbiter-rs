pub mod oapi_consts;
pub mod utils;

#[derive(Debug, Default)]
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
type Vector3 = VECTOR3;

ctype_wrapper!(THRUSTER_HANDLE, usize, ThrusterHandle);
ctype_wrapper!(PROPELLANT_HANDLE, usize, PropellantHandle);
ctype_wrapper!(THGROUP_HANDLE, usize, ThrustGroupHandle);
ctype_wrapper!(OBJHANDLE, usize);
ctype_wrapper!(DWORD, u32);

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct VESSELSTATUS {
	/// position relative to rbody in ecliptic frame [<b>m</b>]
	pub rpos: VECTOR3,

	/// velocity relative to rbody in ecliptic frame [<b>m/s</b>]
	pub rvel: VECTOR3,

	/// rotation velocity about principal axes in ecliptic frame [<b>rad/s</b>]
	pub vrot: VECTOR3,

	/// vessel orientation against ecliptic frame
	pub arot: VECTOR3,

	/// fuel level [0..1]
	fuel: f64,

	/// main/retro engine setting [-1..1]
	eng_main: f64,

	/// hover engine setting [0..1]
	eng_hovr: f64,

	/// handle of reference body
	rbody: OBJHANDLE,

    /// handle of docking or landing target
	base: OBJHANDLE,

    /// index of designated docking or landing port
	port: i32,

    /// \brief flight status indicator
	/// \note
	/// - 0=active (freeflight)
	/// - 1=inactive (landed)
	status: i32,

	/// \brief additional vector parameters
	/// \note
	/// - vdata[0]: contains landing paramters if status == 1:
	///   vdata[0].x = longitude, vdata[0].y = latitude, vdata[0].z = heading of landed vessel
	/// - vdata[1] - vdata[9]: not used
	vdata: [VECTOR3; 10],

	/// additional floating point parameters (not used)
	fdata: [f64; 10],

	/// \brief additional integer and bitflag parameters
	///
	/// \par flag[0]&1:
	///   - 0: ingore eng_main and eng_hovr entries, do not change thruster settings
	///   - 1: set THGROUP_MAIN and THGROUP_RETRO thruster groups from eng_main, and THGROUP_HOVER from eng_hovr.
	/// \par flag[0]&2:
	///   - 0: ignore fuel level, do not change fuel levels
	///   - 1: set fuel level of first propellant resource from fuel
	/// \note flag[1] - flag[9]: not used
	flag: [DWORD; 10],
}
unsafe impl cxx::ExternType for VESSELSTATUS {
    type Id = cxx::type_id!("VESSELSTATUS");
    type Kind = cxx::kind::Trivial;
}
type VesselStatus = VESSELSTATUS;

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
        type THGROUP_HANDLE = crate::ThrustGroupHandle;
        type OBJHANDLE = crate::OBJHANDLE;
        type DWORD = crate::DWORD;
        type THGROUP_TYPE;

        type VESSELSTATUS = crate::VesselStatus;

        fn oapi_create_vessel(name: String, classname: String, status: &VESSELSTATUS) -> OBJHANDLE;

        // VESSEL API
        fn Local2Rel(self: &VesselContext, local: &VECTOR3, rel: &mut VECTOR3);
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

        fn AddMesh(self: &VesselContext, mesh_name: String);
        fn AddMeshWithOffset(self: &VesselContext, mesh_name: String, ofs: &VECTOR3);
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
        ) -> THGROUP_HANDLE;

        fn ClearMeshes(self: &VesselContext);

        fn GetName(self: &VesselContext) -> &str;
        fn GetStatus(self: &VesselContext, status: &mut VESSELSTATUS);
        fn GetPropellantMass(self: &VesselContext, ph: PROPELLANT_HANDLE) -> f64;
        fn GetThrusterGroupLevelByType(self: &VesselContext, thgroup_type: THGROUP_TYPE) -> f64;
 
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
        fn dyn_vessel_consume_buffered_key(
            vessel: &mut BoxDynVessel,
            context: &VesselContext,
            key: DWORD, down: bool, kstate: &str
        ) -> i32;
        unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel);
    }
}

pub use ffi::*;

// Based on https://github.com/dtolnay/cxx/pull/672
use cxx::ExternType;
pub trait OrbiterVessel {
    fn set_class_caps(&mut self, context: &VesselContext);
    fn pre_step(&mut self, context: &VesselContext, sim_t: f64, sim_dt: f64, mjd: f64);
    fn consume_buffered_key(&mut self, context: &VesselContext, key: DWORD, down: bool, kstate: &str) -> i32;
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
fn dyn_vessel_consume_buffered_key(
    vessel: &mut BoxDynVessel,
    context: &VesselContext,
    key: DWORD, down: bool, kstate: &str
) -> i32 {
    (**vessel).consume_buffered_key(context, key, down, kstate)
}

mod macros;

#[allow(dead_code)]
pub mod surveyor;
pub use surveyor::create_rust_spacecraft;
