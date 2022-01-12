/// This module defines the `OrbiterVessel` trait to be implemented by the addon
use crate::{VesselContext, Key, KeyStates, FileHandle};

/// Trait to be implemented by a spacecraft addon module
pub trait OrbiterVessel {
    fn set_class_caps(&mut self, context: &VesselContext, cfg: FileHandle);
    fn on_pre_step(&mut self, _context: &VesselContext, _sim_t: f64, _sim_dt: f64, _mjd: f64) {}
    fn on_post_step(&mut self, _context: &VesselContext, _sim_t: f64, _sim_dt: f64, _mjd: f64) {}
    fn consume_buffered_key(
        &mut self,
        _context: &VesselContext,
        _key: Key,
        _down: bool,
        _kstate: KeyStates,
    ) -> i32 { 0 }
}
