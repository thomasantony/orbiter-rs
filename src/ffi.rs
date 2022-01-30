// This file is included directly from lib.rs as making it into a module created too many hassles
use std::os::raw::c_char;
use std::pin::Pin;

mod vector;

pub use vector::Vector3;
/// Type alias for [VECTOR3](VECTOR3)
pub type VECTOR3 = vector::Vector3;
unsafe impl cxx::ExternType for VECTOR3 {
    type Id = cxx::type_id!("VECTOR3");
    type Kind = cxx::kind::Trivial;
}
 
ctype_wrapper!(THRUSTER_HANDLE, usize, ThrusterHandle); 
ctype_wrapper!(PROPELLANT_HANDLE, usize, PropellantHandle);
ctype_wrapper!(THGROUP_HANDLE, usize, ThrustGroupHandle);
ctype_wrapper!(OBJHANDLE, usize);
ctype_wrapper!(DWORD, u32);

mod io;
pub use io::FileHandle;
pub type FILEHANDLE = io::FileHandle;
unsafe impl cxx::ExternType for FILEHANDLE {
    type Id = cxx::type_id!("FILEHANDLE");
    type Kind = cxx::kind::Trivial;
}

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
    /// Reference Frame
    #[derive(Debug)]
    #[repr(usize)]
    enum REFFRAME {
        /// Global (ecliptic) frame
        #[cxx_name = "FRAME_GLOBAL"]
        Global,
        /// local object frame
        #[cxx_name = "FRAME_LOCAL"]
        Local,
        /// local reference object frame
        #[cxx_name = "FRAME_REFLOCAL"]
        RefLocal,
        /// local horizon frame
        #[cxx_name = "FRAME_HORIZON"]
        Horizon,
    }
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
    /// Altitude Mode
    #[derive(Debug)]
    #[repr(usize)]
    enum AltitudeMode
    {
        /// Altitude over mean radius
        #[cxx_name = "ALTMODE_MEANRAD"]
        MeanRadius,
        /// Altitude over ground
        #[cxx_name = "ALTMODE_GROUND"]
        Ground
    }

    #[derive(Debug)]
    #[repr(usize)]
    enum FileAccessMode {
        /// Read
        #[cxx_name = "FILE_IN"]
        In,
        /// Write
        #[cxx_name = "FILE_OUT"]
        Out,
        /// Write (append)
        #[cxx_name = "FILE_APP"]
        Append,
        /// Read (zero on fail)
        #[cxx_name = "FILE_IN_ZEROONFAIL"]
        In_ZeroOnFail,
    }

    #[derive(Debug)]
    #[repr(usize)]
    enum PathRoot {
        #[cxx_name = "ROOT"]
        Root,
        #[cxx_name = "CONFIG"]
        Config,
        #[cxx_name = "SCENARIOS"]
        Scenarios,
        #[cxx_name = "TEXTURES"]
        Textures,
        #[cxx_name = "TEXTURES2"]
        Textures2,
        #[cxx_name = "MESHES"]
        Meshes,
        #[cxx_name = "MODULES"]
        Modules,
    }


    unsafe extern "C++" {
        include!("include/vessel_context.h");
        #[doc(hidden)]
        type BoxDynVessel = Box<dyn crate::OrbiterVessel>;
        type PtrBoxDynVessel = crate::PtrBoxDynVessel;

        type c_void;
        /// Rust interface to the `VESSELx` abstract classes in Orbiter SDK
        type VesselContext;

        type VECTOR3 = crate::VECTOR3;
        type PROPELLANT_HANDLE = crate::PropellantHandle;
        type THRUSTER_HANDLE = crate::ThrusterHandle;
        type THGROUP_HANDLE = crate::ThrustGroupHandle;
        type OBJHANDLE = crate::OBJHANDLE;
        type DWORD = crate::DWORD;
        type FILEHANDLE = crate::FILEHANDLE;

        type REFFRAME;
        type THGROUP_TYPE;
        type AltitudeMode;

        type VESSELSTATUS = crate::VesselStatus;
        type VESSEL;

        unsafe fn vessel_ovcInit(hvessel: OBJHANDLE, flightmodel: i32, init_fn: fn(Pin<&'static mut VesselContext>)->BoxDynVessel) -> *mut VESSEL;
        // unsafe fn vessel_ovcInit(hvessel: OBJHANDLE, flightmodel: i32, box_vessel: BoxDynVessel) -> *mut VESSEL;
        unsafe fn vessel_ovcExit(vessel: *mut VESSEL);

        /// Create new vessel using Orbiter SDK
        fn oapi_create_vessel(name: String, classname: String, status: &VESSELSTATUS) -> OBJHANDLE;

        // VESSEL API wrappers
        // Some of these have direct counterparts in vessel_context.h
        // Others are using the default implementations from VesselAPI.h (through the VESSEL4 super-class)

        /// Performs a transformation from local vessel coordinates to the ecliptic frame centered at the vessel's reference body
        fn Local2Rel(self: &VesselContext, local: &VECTOR3, rel: &mut VECTOR3);
        /// Performs a transformation from global (ecliptic) to local vessel coordinates
        fn Global2Local(self: &VesselContext, global: &VECTOR3, local: &mut VECTOR3);
        /// Performs a transformation from local vessel to global (ecliptic) coordinates
        fn Local2Global(self: &VesselContext, local: &VECTOR3, global: &mut VECTOR3);

        /// Set the vessel's mean radius
        ///
        /// The size should correspond to the vessel's visual representation, for example the mesh used to show the vessel in the simulation window.
        /// The size parameter is used by Orbiter to determine the camera distance at which the vessel is within visual
        /// range of the observer camera. It is also used for calculating various physical parameters.
        /// If SetSize is not called during the vessel setup, the value from the Size entry in the vessel's configuration file
        /// is used.
        fn SetSize(self: &VesselContext, size: f64);
        /// Set the vessel's mass-normalised principal moments of inertia (PMI)
        fn SetPMI(self: &VesselContext, pmi: &VECTOR3);
        /// Set the vessel's empty mass (excluding propellants)
        fn SetEmptyMass(self: &VesselContext, empty_mass: f64);
        /// Set the camera position for internal (cockpit) view.
        /// 
        /// # Arguments
        /// * `camera_offset` - Camera offset in vessel coordinates
        fn SetCameraOffset(self: &VesselContext, camera_offset: &VECTOR3);
        /// Defines the three points defining the vessel's ground contact plane
        fn SetTouchdownPoints(self: &VesselContext, pt1: &VECTOR3, pt2: &VECTOR3, pt3: &VECTOR3);
        /// Reset the force direction of a thruster
        fn SetThrusterDir(self: &VesselContext, th: THRUSTER_HANDLE, dir: &VECTOR3);
        /// Set thrust level for a thruster
        fn SetThrusterLevel(self: &VesselContext, th: THRUSTER_HANDLE, level: f64);
        /// Set the thrust level of a thruster for the current time step only
        fn SetThrusterLevel_SingleStep(self: &VesselContext, th: THRUSTER_HANDLE, level: f64);

        /// Load a mesh definition for the vessel from a file
        fn AddMesh(self: &VesselContext, mesh_name: String);
        /// Load a mesh definition for the vessel from a file displaced by offset `ofs`
        /// 
        /// # Arguments
        /// * `mesh_name` - name of the mesh file (without extension)
        /// * `ofs` - a displacement vector which describes the offset of the mesh origin against the vessel origin
        fn AddMeshWithOffset(self: &VesselContext, mesh_name: String, ofs: &VECTOR3);
        /// Add an exhaust render definition for a thruster
        fn AddExhaust(self: &VesselContext, th: THRUSTER_HANDLE, lscale: f64, wscale: f64)
            -> usize;

        /// Create a new propellant resource ("fuel tank")
        /// 
        /// Propellant resources are a component of the vessel's propulsion system. They can hold propellants and distribute
        /// them to connected engines to generate thrust
        fn CreatePropellantResource(self: &VesselContext, mass: f64) -> PROPELLANT_HANDLE;
        /// Add a logical thruster definition for the vessel
        fn CreateThruster(
            self: &VesselContext,
            pos: &VECTOR3,
            dir: &VECTOR3,
            maxth0: f64,
            ph: PROPELLANT_HANDLE,
            isp: f64,
        ) -> THRUSTER_HANDLE;
        /// Combine thrusters into a logical thruster group
        fn CreateThrusterGroup(
            self: &VesselContext,
            thrusters: &[THRUSTER_HANDLE],
            thgroup_type: THGROUP_TYPE,
        ) -> THGROUP_HANDLE;

        /// Remove all mesh definitions for the vessel
        fn ClearMeshes(self: &VesselContext);

        /// Returns the vessel's name
        fn GetName(self: &VesselContext) -> &str;
        /// Returns the vessel's current status parameters in a [VesselStatus] structure
        fn GetStatus(self: &VesselContext, status: &mut VESSELSTATUS);
        /// Returns the current mass of a propellant resource specified by `ph`
        fn GetPropellantMass(self: &VesselContext, ph: PROPELLANT_HANDLE) -> f64;
        /// Get angular velocity (in rad/s) of the spacecraft around its principal axes and store it in `a_vel`
        fn GetAngularVel(self: &VesselContext, a_vel: &mut VECTOR3);
        /// Returns the vessel's true "airspeed" vector
        /// 
        /// This method returns the true airspeed vector in the requested frame of reference. The ground airvector is
        /// defined as the vessel's velocity vector with respect to the surrounding freestream air flow.
        /// If the vessel is not within an a planetary atmosphere, the returned vector is equal to the groundspeed vector
        fn GetAirspeedVector(self: &VesselContext, ref_frame: REFFRAME, airspeed: &mut VECTOR3) -> bool;
        /// Returns thrust force vector in local vessel coordinates
        fn GetThrustVector(self: &VesselContext, thrust_vec: &mut VECTOR3) -> bool;
        /// Returns the vessel's current total propellant mass
        fn GetTotalPropellantMass(self: &VesselContext) -> f64;
        /// Returns the mean thrust level for a default thruster group type
        #[rust_name = "GetThrusterGroupLevelByType"]
        fn GetThrusterGroupLevel(self: &VesselContext, thgroup_type: THGROUP_TYPE) -> f64;
        /// Returns the mean thrust level for a default thruster group specified by `th`
        #[rust_name = "GetThrusterGroupLevel"]
        fn GetThrusterGroupLevel(self: &VesselContext, th: THGROUP_HANDLE) -> f64;
        /// Returns a flag indicating contact with a planetary surface
        fn GroundContact(self: &VesselContext) -> bool;

        /// Returns a handle to the surface reference object (planet or moon)
        fn GetSurfaceRef(self: &VesselContext) -> OBJHANDLE;

        /// Returns the elevation of the surface at the vessel's current longitude/latitude above the reference radius
        fn GetSurfaceElevation(self: &VesselContext) -> f64;

        /// Returns altitude above mean ellipsoid
        fn GetAltitude(self: &VesselContext) -> f64;

        /// Returns the vessel's current velocity relative to another object
        /// 
        /// Results are returned in the ecliptic frame (ecliptic and equinox of J2000.0). 
        fn GetRelativeVel(self: &VesselContext, href: OBJHANDLE, rel_vel: &mut VECTOR3);

        /// Pass a line read from a scenario file to Orbiter for default processing
        unsafe fn ParseScenarioLineEx(self: &VesselContext, line: *mut c_char, status: *mut c_void);

        /// Print message to lower-left corner of screen. For debugging purposes only!
        fn ODebug(s: String);

        type FileAccessMode;
        type PathRoot;

        unsafe fn oapiReadItem_string(f: FILEHANDLE, item: *mut c_char, val: *mut c_char) -> bool;
        unsafe fn oapiReadItem_float(f: FILEHANDLE, item: *mut c_char, val: &mut f64) -> bool;
        unsafe fn oapiReadItem_int(f: FILEHANDLE, item: *mut c_char, val: &mut i32) -> bool;
        unsafe fn oapiReadItem_bool(f: FILEHANDLE, item: *mut c_char, val: &mut bool) -> bool;
        unsafe fn oapiReadItem_vec(f: FILEHANDLE, item: *mut c_char, val: &mut VECTOR3) -> bool;

        unsafe fn oapiWriteItem_string(f: FILEHANDLE, item: *mut c_char, val: *mut c_char);
        unsafe fn oapiWriteItem_float(f: FILEHANDLE, item: *mut c_char, val: f64);
        unsafe fn oapiWriteItem_int(f: FILEHANDLE, item: *mut c_char, val: i32);
        unsafe fn oapiWriteItem_bool(f: FILEHANDLE, item: *mut c_char, val: bool);
        unsafe fn oapiWriteItem_vec(f: FILEHANDLE, item: *mut c_char, val: &VECTOR3);

        unsafe fn oapiWriteScenario_string(scn: FILEHANDLE, item: *mut c_char, val: *mut c_char);
        unsafe fn oapiWriteScenario_float(scn: FILEHANDLE, item: *mut c_char, val: f64);
        unsafe fn oapiWriteScenario_int(scn: FILEHANDLE, item: *mut c_char, val: i32);
        unsafe fn oapiWriteScenario_vec(scn: FILEHANDLE, item: *mut c_char, val: &VECTOR3);

        unsafe fn oapiReadScenario_nextline(scn: FILEHANDLE, line: &mut *mut c_char) -> bool;
        unsafe fn oapiWriteLine(f: FILEHANDLE, line: *mut c_char);

        unsafe fn oapiOpenFile(
            fname: *const c_char,
            mode: FileAccessMode,
            root: PathRoot,
        ) -> FILEHANDLE;
        fn oapiCloseFile(f: FILEHANDLE, mode: FileAccessMode);

        /// Writes a line to the Orbiter log file (orbiter.log) in the main orbiter directory
        unsafe fn oapiWriteLog(line: *mut c_char);
    }
    extern "Rust" {
        fn dyn_vessel_set_class_caps(vessel: &mut BoxDynVessel, cfg: &FILEHANDLE);
        fn dyn_vessel_pre_step(
            vessel: &mut BoxDynVessel,
            sim_t: f64,
            sim_dt: f64,
            mjd: f64,
        );
        fn dyn_vessel_post_step(
            vessel: &mut BoxDynVessel,
            sim_t: f64,
            sim_dt: f64,
            mjd: f64,
        );
        unsafe fn dyn_vessel_consume_buffered_key(
            vessel: &mut BoxDynVessel,
            key: DWORD,
            down: bool,
            kstate: *mut c_char,
        ) -> i32;
        unsafe fn dyn_vessel_drop_in_place(ptr: PtrBoxDynVessel);
        unsafe fn dyn_vessel_load_state_ex(
            vessel: &mut BoxDynVessel,
            scn: FILEHANDLE,
            status: *mut c_void,
            sdk_vessel: Pin<&mut VesselContext>
        );
        fn dyn_vessel_save_state(vessel: &mut BoxDynVessel, scn: FILEHANDLE);
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
fn dyn_vessel_set_class_caps(vessel: &mut Box<dyn OrbiterVessel>, cfg: &FileHandle) {
    (**vessel).set_class_caps(cfg);
}
fn dyn_vessel_pre_step(
    vessel: &mut Box<dyn OrbiterVessel>,
    sim_t: f64,
    sim_dt: f64,
    mjd: f64,
) {
    (**vessel).on_pre_step(sim_t, sim_dt, mjd);
}
fn dyn_vessel_post_step(
    vessel: &mut Box<dyn OrbiterVessel>,
    sim_t: f64,
    sim_dt: f64,
    mjd: f64,
) {
    (**vessel).on_post_step(sim_t, sim_dt, mjd);
}
unsafe fn dyn_vessel_consume_buffered_key(
    vessel: &mut BoxDynVessel,
    key: DWORD,
    down: bool,
    kstate: *mut c_char,
) -> i32 {
    let kstate = crate::KeyStates::from(kstate);
    (**vessel).consume_buffered_key(crate::Key::from(key.0 as u8), down, kstate)
}
unsafe fn dyn_vessel_load_state_ex(
    vessel: &mut BoxDynVessel,
    scn: FILEHANDLE,
    status: *mut ffi::c_void,
    sdk_vessel: Pin<&mut VesselContext>)
{
    let mut line: *mut c_char = std::ptr::null_mut();
    loop {
        let res = ffi::oapiReadScenario_nextline(scn, &mut line);
        if !res {
            break;
        }

        let line_str = std::ffi::CStr::from_ptr(line).to_string_lossy();
        if !vessel.on_load_param(&line_str)
        {
            sdk_vessel.ParseScenarioLineEx(line, status);
        }
    }
}

fn dyn_vessel_save_state(
    vessel: &mut BoxDynVessel,
    scn: FILEHANDLE
) {
    vessel.on_save_state(&scn);
}
pub use ffi::VesselContext;
impl std::fmt::Debug for VesselContext {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("VesselContext").finish()
    }
}
#[doc(hidden)]
pub use ffi::BoxDynVessel;

pub use ffi::oapi_create_vessel;
pub use ffi::ODebug;

pub use ffi::REFFRAME as ReferenceFrame;
/// Type alias for [THGROUP_TYPE]
pub use ffi::THGROUP_TYPE as ThrusterGroupType;

/// A wrapper over the FFI interface to the Orbiter SDK
/// This is passed to the init() function in the `[init_vessel!]` macro
pub type SDKVessel = std::pin::Pin<&'static mut VesselContext>;
