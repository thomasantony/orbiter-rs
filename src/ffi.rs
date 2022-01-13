// This file is included directly from lib.rs as making it into a module created too many hassles
use std::os::raw::c_char;

/// Rust binding for `VECTOR3`
#[derive(Debug, Default)]
#[repr(C)]
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
/// Type alias for [VECTOR3](VECTOR3)
pub type Vector3 = VECTOR3;
 
ctype_wrapper!(THRUSTER_HANDLE, usize, ThrusterHandle); 
ctype_wrapper!(PROPELLANT_HANDLE, usize, PropellantHandle);
ctype_wrapper!(THGROUP_HANDLE, usize, ThrustGroupHandle);
ctype_wrapper!(FILEHANDLE, usize, FileHandle);
ctype_wrapper!(OBJHANDLE, usize);
ctype_wrapper!(DWORD, u32);

/// Binding for OrbiterSDK's `VESSELSTATUS` struct
#[repr(C)]
#[derive(Debug, Default)]
pub struct VESSELSTATUS {
    /// Position relative to rbody in ecliptic frame \[**m**\]
    pub rpos: VECTOR3,

    /// Velocity relative to rbody in ecliptic frame \[**m/s**\]
    pub rvel: VECTOR3,

    /// Rotation velocity about principal axes in ecliptic frame \[**rad/s**\]
    pub vrot: VECTOR3,

    /// Vessel orientation against ecliptic frame
    pub arot: VECTOR3,

    /// Fuel level. Between 0 and 1.
    pub fuel: f64,

    /// Main/retro engine setting. Between -1 and 1.
    pub eng_main: f64,

    /// Hover engine setting. Between 0 and 1.
    pub eng_hovr: f64,

    /// Handle of reference body
    pub rbody: OBJHANDLE,

    /// Handle of docking or landing target
    pub base: OBJHANDLE,

    /// Index of designated docking or landing port
    pub port: i32,

    /// Flight status indicator
    /// 
    /// - 0 = active (freeflight)
    /// - 1 = inactive (landed)
    pub status: i32,

    /// Additional vector parameters
    /// 
    /// - `vdata[0]`: contains landing parameters 
    /// 
    ///    if `status` is equal to 1, `vdata[0]` contains the longitude, latitude, and heading of landed vessel
    /// 
    /// - `vdata[1]` - `vdata[9]`: not used
    pub vdata: [VECTOR3; 10],

    /// additional floating point parameters (not used)
    pub fdata: [f64; 10],

    /// Additional integer and bitflag parameters
    ///
    /// - `flag[0] & 1`:
    ///   - 0: ingore eng_main and eng_hovr entries, do not change thruster settings
    ///   - 1: set [ThrusterGroupType::Main] and [ThrusterGroupType::Retro] thruster groups from `eng_main`, and [ThrusterGroupType::Hover] from `eng_hovr`.
    ///
    /// - `flag[0] & 2`:
    ///   - 0: ignore fuel level, do not change fuel levels
    ///   - 1: set fuel level of first propellant resource from fuel
    /// 
    /// - `flag[1]` - `flag[9]`: not used
    pub flag: [DWORD; 10],
}
unsafe impl cxx::ExternType for VESSELSTATUS {
    type Id = cxx::type_id!("VESSELSTATUS");
    type Kind = cxx::kind::Trivial;
}
/// Type alias for [VESSELSTATUS]
pub type VesselStatus = VESSELSTATUS;

#[doc(hidden)]
#[cxx::bridge]
pub mod ffi {
    /// Thruster Group Type
    #[derive(Debug)]
    #[repr(usize)]
    enum THGROUP_TYPE {
        /// Main Thrusters
        #[cxx_name = "THGROUP_MAIN"]
        Main,
        /// Retro Thrusters
        #[cxx_name = "THGROUP_RETRO"]
        Retro,
        /// Hover Thrusters
        #[cxx_name = "THGROUP_HOVER"]
        Hover,
        /// Rotation: Pitch Up
        #[cxx_name = "THGROUP_ATT_PITCHUP"]
        AttPitchup,
        /// Rotation: Pitch Down
        #[cxx_name = "THGROUP_ATT_PITCHDOWN"]
        AttPitchdown,
        /// Rotation: Yaw Left
        #[cxx_name = "THGROUP_ATT_YAWLEFT"]
        AttYawleft,
        /// Rotation: Yaw Right
        #[cxx_name = "THGROUP_ATT_YAWRIGHT"]
        AttYawright,
        /// Rotation: Bank Left
        #[cxx_name = "THGROUP_ATT_BANKLEFT"]
        AttBankleft,
        /// Rotation: Bank Right
        #[cxx_name = "THGROUP_ATT_BANKRIGHT"]
        AttBankright,
        /// Translation: Move Right
        #[cxx_name = "THGROUP_ATT_RIGHT"]
        AttRight,
        /// Translation: Move Left
        #[cxx_name = "THGROUP_ATT_LEFT"]
        AttLeft,
        /// Translation: Move Up
        #[cxx_name = "THGROUP_ATT_UP"]
        AttUp,
        /// Translation: Move Down
        #[cxx_name = "THGROUP_ATT_DOWN"]
        AttDown,
        /// Translation: Move Forward
        #[cxx_name = "THGROUP_ATT_FORWARD"]
        AttForward,
        /// Translation: Move Back
        #[cxx_name = "THGROUP_ATT_BACK"]
        AttBack,
        /// User-Defined Thruster Group
        #[cxx_name = "THGROUP_USER"]
        User = 0x40,
    }
    unsafe extern "C++" {
        include!("include/vessel_context.h");
        #[doc(hidden)]
        type BoxDynVessel = Box<dyn crate::OrbiterVessel>;
        type PtrBoxDynVessel = crate::PtrBoxDynVessel;

        /// Rust interface to the `VESSELx` abstract classes in Orbiter SDK
        type VesselContext;

        type VECTOR3 = crate::VECTOR3;
        type PROPELLANT_HANDLE = crate::PropellantHandle;
        type THRUSTER_HANDLE = crate::ThrusterHandle;
        type THGROUP_HANDLE = crate::ThrustGroupHandle;
        type OBJHANDLE = crate::OBJHANDLE;
        type DWORD = crate::DWORD;
        type FILEHANDLE = crate::FILEHANDLE;
        type THGROUP_TYPE;

        type VESSELSTATUS = crate::VesselStatus;
        type VESSEL;

        unsafe fn vessel_ovcInit(hvessel: OBJHANDLE, flightmodel: i32, box_vessel: BoxDynVessel) -> *mut VESSEL;
        unsafe fn vessel_ovcExit(vessel: *mut VESSEL);

        /// Create new vessel using Orbiter SDK
        fn oapi_create_vessel(name: String, classname: String, status: &VESSELSTATUS) -> OBJHANDLE;

        // VESSEL API wrappers
        // Some of these have direct counterparts in vessel_context.h
        // Others are using the default implementations from VesselAPI.h (through the VESSEL4 super-class)
        fn Local2Rel(self: &VesselContext, local: &VECTOR3, rel: &mut VECTOR3);

        fn SetSize(self: &VesselContext, size: f64);
        fn SetPMI(self: &VesselContext, pmi: &VECTOR3);
        fn SetEmptyMass(self: &VesselContext, empty_mass: f64);
        fn SetCameraOffset(self: &VesselContext, camera_offset: &VECTOR3);
        fn SetTouchdownPoints(self: &VesselContext, pt1: &VECTOR3, pt2: &VECTOR3, pt3: &VECTOR3);
        fn SetThrusterDir(self: &VesselContext, th: THRUSTER_HANDLE, dir: &VECTOR3);
        fn SetThrusterLevel(self: &VesselContext, th: THRUSTER_HANDLE, level: f64);

        fn AddMesh(self: &VesselContext, mesh_name: String);
        fn AddMeshWithOffset(self: &VesselContext, mesh_name: String, ofs: &VECTOR3);
        fn AddExhaust(self: &VesselContext, th: THRUSTER_HANDLE, lscale: f64, wscale: f64)
            -> usize;

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
        /// Get angular velocity (in rad/s) of the spacecraft around its principal axes and store it in `a_vel`
        fn GetAngularVel(self: &VesselContext, a_vel: &mut VECTOR3);

        #[rust_name = "GetThrusterGroupLevelByType"]
        fn GetThrusterGroupLevel(self: &VesselContext, thgroup_type: THGROUP_TYPE) -> f64;
        #[rust_name = "GetThrusterGroupLevel"]
        fn GetThrusterGroupLevel(self: &VesselContext, th: THGROUP_HANDLE) -> f64;

        /// Print message to lower-left corner of screen. For debugging purposes only!
        fn ODebug(s: &str);
    }
    extern "Rust" {
        fn dyn_vessel_set_class_caps(vessel: &mut BoxDynVessel, context: &VesselContext, cfg: FILEHANDLE);
        fn dyn_vessel_pre_step(
            vessel: &mut BoxDynVessel,
            context: &VesselContext,
            sim_t: f64,
            sim_dt: f64,
            mjd: f64,
        );
        fn dyn_vessel_post_step(
            vessel: &mut BoxDynVessel,
            context: &VesselContext,
            sim_t: f64,
            sim_dt: f64,
            mjd: f64,
        );
        unsafe fn dyn_vessel_consume_buffered_key(
            vessel: &mut BoxDynVessel,
            context: &VesselContext,
            key: DWORD,
            down: bool,
            kstate: *mut c_char,
        ) -> i32;
        unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel);
    }
}

/// The following is a workaround for passing Boxed trait objects to C++ code
/// and then calling trait methods on them
/// 
/// Based on [https://github.com/dtolnay/cxx/pull/672](https://github.com/dtolnay/cxx/pull/672)
unsafe impl ExternType for Box<dyn OrbiterVessel> {
    type Id = cxx::type_id!("BoxDynVessel");
    type Kind = cxx::kind::Trivial;
}
#[doc(hidden)]
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
fn dyn_vessel_set_class_caps(vessel: &mut Box<dyn OrbiterVessel>, context: &VesselContext, cfg: FileHandle) {
    (**vessel).set_class_caps(context, cfg);
}
fn dyn_vessel_pre_step(
    vessel: &mut Box<dyn OrbiterVessel>,
    context: &VesselContext,
    sim_t: f64,
    sim_dt: f64,
    mjd: f64,
) {
    (**vessel).on_pre_step(context, sim_t, sim_dt, mjd);
}
fn dyn_vessel_post_step(
    vessel: &mut Box<dyn OrbiterVessel>,
    context: &VesselContext,
    sim_t: f64,
    sim_dt: f64,
    mjd: f64,
) {
    (**vessel).on_post_step(context, sim_t, sim_dt, mjd);
}
unsafe fn dyn_vessel_consume_buffered_key(
    vessel: &mut BoxDynVessel,
    context: &VesselContext,
    key: DWORD,
    down: bool,
    kstate: *mut c_char,
) -> i32 {
    let kstate = crate::KeyStates::from(kstate);
    (**vessel).consume_buffered_key(context, crate::Key::from(key.0 as u8), down, kstate)
}

pub use ffi::VesselContext;

#[doc(hidden)]
pub use ffi::BoxDynVessel;

pub use ffi::ODebug;
pub use ffi::oapi_create_vessel;
/// Type alias for [THGROUP_TYPE]
pub use ffi::THGROUP_TYPE as ThrusterGroupType;
