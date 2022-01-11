/// Helper macro for defining Vector3 objects
#[macro_export]
macro_rules! _V {
    ($x:expr, $y:expr, $z:expr) => {
        &$crate::Vector3::new($x, $y, $z)
    };
}
/// Macro for defining ctype wrapper
/// Adapted from https://github.com/dtolnay/cxx/issues/254#issuecomment-747860504 by Adrian Taylor
#[macro_export]
macro_rules! ctype_wrapper {
    ($r:ident, $c:ty) => {
        /// Newtype wrapper for a `$c`
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
        /// Newtype wrapper for a `$c`
        #[derive(Debug, Eq, Clone, PartialEq, Hash, Default, Copy)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $r(pub $c);
        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($r);
            type Kind = cxx::kind::Trivial;
        }
        pub type $nice_name = $r;
    };
}

/// Helper macro for defining entry point into a Vessel addon
/// Inspired by emgre's orbiter-rs 
/// https://github.com/emgre/orbiter-rs/blob/107068c6e66564b9dff86c8b964515da9771a3af/orbiter/src/lib.rs#L37
#[macro_export]
macro_rules! init_vessel {
    (fn init($hvessel_ident:ident :OBJHANDLE, $flightmodel_ident:ident :i32) $body_init:block fn exit() $body_exit:block) => {
        #[no_mangle]
        pub extern "C" fn ovcInit (hvessel: $crate::OBJHANDLE, flightmodel: i32) -> *mut $crate::VESSEL
        {
            let ($hvessel_ident, $flightmodel_ident) = (hvessel, flightmodel);
            let spacecraft = {
                $body_init
            };
            unsafe { $crate::vessel_ovcInit(hvessel, flightmodel, Box::new(spacecraft)) }
        }
        #[no_mangle]
        pub extern "C" fn ovcExit (vessel: *mut $crate::VESSEL)
        {
            unsafe { $crate::vessel_ovcExit(vessel); }
        }
    };
}
