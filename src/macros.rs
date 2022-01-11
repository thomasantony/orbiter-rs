/// Helper macro for defining the struct to be used by the addon
#[macro_export]
macro_rules! make_orbiter_vessel {
    ($vessel:expr) => {
        #[no_mangle]
        pub fn create_rust_spacecraft() -> Box<dyn OrbiterVessel> {
            Box::new($vessel)
        }
    };
}
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
        pub struct $r(pub usize);
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
        pub struct $r(pub usize);
        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($r);
            type Kind = cxx::kind::Trivial;
        }
        type $nice_name = $r;
    };
}
