use orbiter_rs::{debug_string, init_vessel, FileHandle, OrbiterVessel, SDKVessel};
pub struct MinimalPB {
    ctx: SDKVessel,
}
impl MinimalPB {
    pub fn new(vessel: SDKVessel) -> Self {
        Self { ctx: vessel }
    }
}
impl OrbiterVessel for MinimalPB {
    fn set_class_caps(&mut self, _cfg: FileHandle) {
        self.ctx.SetSize(1.0);
        self.ctx.AddMesh("ShuttlePB".into());
    }
    fn on_pre_step(&mut self, sim_t: f64, _sim_dt: f64, _mjd: f64) {
        debug_string!("Hello world! sim_t: {:.2}", sim_t);
    }
}

init_vessel!(
    fn init(vessel) {
        MinimalPB::new(vessel)
    }
    fn exit() {}
);
