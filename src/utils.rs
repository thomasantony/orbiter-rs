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
}
