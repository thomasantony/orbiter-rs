#[macro_export]
macro_rules! ctype_wrapper {
    ($r:ident, $t:expr, $c:expr) => {
        /// Newtype wrapper for a `$c`
        #[derive(Debug, Eq, Clone, PartialEq, Hash)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $r(pub $t);

        unsafe impl cxx::ExternType for $r {
            type Id = cxx::type_id!($c);
            type Kind = cxx::kind::Trivial;
        }
    };
}
