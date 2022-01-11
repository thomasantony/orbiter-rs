/// This module defines the `OrbiterVessel` trait to be implemented by the addon
use crate::ffi::{VesselContext, DWORD};

pub trait OrbiterVessel {
    fn set_class_caps(&mut self, context: &VesselContext);
    fn pre_step(&mut self, context: &VesselContext, sim_t: f64, sim_dt: f64, mjd: f64);
    fn consume_buffered_key(
        &mut self,
        context: &VesselContext,
        key: DWORD,
        down: bool,
        kstate: [u8; crate::consts::LKEY_COUNT],
    ) -> i32;
}
