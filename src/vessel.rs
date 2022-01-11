/// This module defines the `OrbiterVessel` trait to be implemented by the addon
use crate::ffi::{VesselContext, DWORD};

pub trait OrbiterVessel {
    fn set_class_caps(&mut self, context: &VesselContext);
    fn pre_step(&mut self, _context: &VesselContext, _sim_t: f64, _sim_dt: f64, _mjd: f64) {}
    fn consume_buffered_key(
        &mut self,
        _context: &VesselContext,
        _key: DWORD,
        _down: bool,
        _kstate: [u8; crate::consts::LKEY_COUNT],
    ) -> i32 { 0 }
}
