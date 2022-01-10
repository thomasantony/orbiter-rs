// extern crate proc_macro;

// #[proc_macro_derive(OrbiterVessel)]
// pub fn make_vessel_shim(_item: TokenStream) -> TokenStream {
//     "fn answer() -> u32 { 42 }".parse().unwrap()
// }
#[macro_export]
macro_rules! make_orbiter_vessel {
    ($vessel:expr) => {
        #[no_mangle]
        fn create_rust_spacecraft() -> Box<dyn OrbiterVessel> {
            Box::new($vessel)
        }
    }
}
#[macro_export]
macro_rules! _V {
    ($x:expr, $y:expr, $z:expr) => {
        & $crate::Vector3::new($x, $y, $z)
    }
}
