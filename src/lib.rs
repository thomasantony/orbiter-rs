use cxx::ExternType;

pub mod consts;
mod macros;

mod vessel;
pub use vessel::OrbiterVessel;

// FFI interface to orbiter.rs
include!("ffi.rs");

/// Spacecraft implementation
mod surveyor;

#[macro_export]
macro_rules! init_vessel {
    (fn init($hvessel_ident:ident :OBJHANDLE, $flightmodel_ident:ident :i32) $body_init:block fn exit() $body_exit:block) => {
        #[no_mangle]
        pub extern "C" fn ovcInit (hvessel: crate::OBJHANDLE, flightmodel: i32) -> *mut crate::VESSEL
        {
            let ($hvessel_ident, $flightmodel_ident) = (hvessel, flightmodel);
            let spacecraft = {
                $body_init
            };
            unsafe { crate::vessel_ovcInit(hvessel, flightmodel, Box::new(spacecraft)) }
        }
        #[no_mangle]
        pub extern "C" fn ovcExit (vessel: *mut crate::VESSEL)
        {
            unsafe { crate::vessel_ovcExit(vessel); }
        }
    };
}
