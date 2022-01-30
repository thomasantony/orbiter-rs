/// This module defines the `OrbiterVessel` trait to be implemented by the addon
use crate::{FileHandle, Key, KeyStates};

/// Trait to be implemented by a spacecraft addon module
pub trait OrbiterVessel {
    fn set_class_caps(&mut self, cfg: &FileHandle);
    fn on_pre_step(&mut self, _sim_t: f64, _sim_dt: f64, _mjd: f64) {}
    fn on_post_step(&mut self, _sim_t: f64, _sim_dt: f64, _mjd: f64) {}
    fn consume_buffered_key(&mut self, _key: Key, _down: bool, _kstate: KeyStates) -> i32 {
        0
    }
    /// Triggered for each line of vehicle parameters when a scenario is loaded
    /// Return true to indicate that it was a custom value parsed by the module
    /// Returning false will pass the parameter to Orbiter for default processing
    fn on_load_param(&mut self, _param_data: &str) -> bool {
        false
    }
    /// Triggered when a scenario containing the vessel is saved. This is used for 
    /// persisting any custom vehicle parameters to the scenario
    /// 
    /// Use the FileHandle::write_scenario_*() methods here
    fn on_save_state(&mut self, _scn: &FileHandle){}
}
