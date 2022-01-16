/// Helper macro for defining [`Vector3`](super::VECTOR3) objects
#[macro_export]
macro_rules! V {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::Vector3::new($x, $y, $z)
    };
}
/// Macro for defining ctype wrappers
/// 
/// Adapted from [comment](https://github.com/dtolnay/cxx/issues/254#issuecomment-747860504) by Adrian Taylor
#[macro_export]
macro_rules! ctype_wrapper {
    ($r:ident, $c:ty) => {
        
        #[doc = "Newtype wrapper for `"]
        #[doc = stringify!($r)]
        #[doc = "` as a [`"]
        #[doc = stringify!($c)]
        #[doc = "`]"]
        #[derive(Debug, Eq, Clone, PartialEq, Hash, Default, Copy)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $r(pub $c);
        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($r);
            type Kind = cxx::kind::Trivial;
        }
    };
    ($r:ident, $c:ty, $nice_name:ident) => {
        #[doc = "Newtype wrapper for `"]
        #[doc = stringify!($r)]
        #[doc = "` as a ["]
        #[doc = stringify!($c)]
        #[doc = "]"]
        #[derive(Debug, Eq, Clone, PartialEq, Hash, Default, Copy)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $r(pub $c);
        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($r);
            type Kind = cxx::kind::Trivial;
        }
        #[doc = "Type alias for ["]
        #[doc = stringify!($r)]
        #[doc = "]"]
        pub type $nice_name = $r;
    };
}

/// Helper macro for defining entry point into a Vessel addon
/// 
/// Inspired by emgre's [orbiter-rs](https://github.com/emgre/orbiter-rs/blob/107068c6e66564b9dff86c8b964515da9771a3af/orbiter/src/lib.rs#L37)
/// 
/// The macro should contain two function blocks `init()` and `exit()`. The `init` function takes two arguments,
/// and [`OBJHANDLE`](super::OBJHANDLE) and an [`i32`] and returns an instance of a struct that implements the `[OrbiterVessel]` trait. This function is called each time a scenario containing the vessel is loaded.
/// 
/// The `exit` function is called at the end of a simulation session and can be used to perform cleanup functions.
/// 
/// Example:
/// ```no_run
/// init_vessel!(
///    fn init(_h_vessel: OBJHANDLE, _flight_model: i32) -> Surveyor 
///    {
///        Surveyor::default()
///    }
///    fn exit() {}
/// );
/// ```
#[macro_export]
macro_rules! init_vessel {
    (fn init($hvessel_ident:ident :OBJHANDLE, $flightmodel_ident:ident :i32) -> $spacecraft_type:ty $body_init:block fn exit() $body_exit:block) => {
        #[no_mangle]
        pub extern "C" fn ovcInit (hvessel: $crate::OBJHANDLE, flightmodel: i32) -> *mut $crate::ffi::VESSEL
        {
            let ($hvessel_ident, $flightmodel_ident) = (hvessel, flightmodel);
            let spacecraft: $spacecraft_type = {
                $body_init
            };
            unsafe { $crate::ffi::vessel_ovcInit(hvessel, flightmodel, Box::new(spacecraft)) }
        }
        #[no_mangle]
        pub extern "C" fn ovcExit (vessel: *mut $crate::ffi::VESSEL)
        {
            $body_exit
            unsafe { $crate::ffi::vessel_ovcExit(vessel); }
        }
    };
}
