
/// Surveyor spacecraft definition using the SDK
use crate::{make_orbiter_vessel, _V, OrbiterVessel, SpacecraftWrapper, debugLog};


const VERNIER_PROP_MASS:f64 = 70.98;
const VERNIER_ISP:f64 = 3200.0;
const VERNIER_THRUST:f64 = 463.0;
const VERNIER_RAD:f64 = 0.86;
const VERNIER_Z:f64 = -0.5;

const RCS_PROP_MASS:f64 = 2.0;
const RCS_ISP:f64 = 630.0;
const RCS_THRUST:f64 = 0.25;
const RCS_RAD:f64 = 1.0;
const RCS_STA:f64 = -0.5;
const RCS_SPACE:f64 = 0.1;

const RETRO_PROP_MASS:f64 = 560.64;
const RETRO_THRUST:f64 = 39140.0;
const RETRO_BURNTIME:f64 = 40.5;
const RETRO_ITOT:f64 = RETRO_THRUST * RETRO_BURNTIME;
const RETRO_ISP:f64 = RETRO_ITOT / RETRO_PROP_MASS;
const RETRO_STA:f64 = -0.75;

const LANDER_EMPTY_MASS:f64 = 289.10; //Basic bus plus payload minus AMR minus retro case
const RETRO_EMPTY_MASS:f64 = 64.88;
const AMR_MASS:f64 = 3.82;

const LEG_RAD:f64 = 1.5;
const LEG_STA:f64 = -0.6;

pub struct RustSpacecraft
{
    th_vernier: Vec<usize>,
}
impl RustSpacecraft {
    pub fn new() -> Self {
        Self {
            th_vernier: Vec::new()
        }
    }
}
impl OrbiterVessel for RustSpacecraft {
    fn set_class_caps(&mut self, context: &SpacecraftWrapper) {
        context.SetSize(1.0);
        context.SetPMI(_V!(0.50, 0.50, 0.50));

        let ph_vernier = context.CreatePropellantResource (VERNIER_PROP_MASS);
        self.th_vernier.push(context.CreateThruster(_V!(                          0.0 * VERNIER_RAD,  1.0 * VERNIER_RAD, VERNIER_Z), _V!(0.0, 0.0, 1.0), VERNIER_THRUST, ph_vernier, VERNIER_ISP));
        self.th_vernier.push(context.CreateThruster(_V!(( 60.0f64).to_radians().sin() * VERNIER_RAD, -0.5 * VERNIER_RAD, VERNIER_Z), _V!(0.0, 0.0, 1.0), VERNIER_THRUST, ph_vernier, VERNIER_ISP));
        self.th_vernier.push(context.CreateThruster(_V!((-60.0f64).to_radians().sin() * VERNIER_RAD, -0.5 * VERNIER_RAD, VERNIER_Z), _V!(0.0, 0.0, 1.0), VERNIER_THRUST, ph_vernier, VERNIER_ISP));
        context.CreateThrusterGroup(&self.th_vernier);
        for th in self.th_vernier.iter() {
            context.AddExhaust(*th, 1.0, 0.1);
        }

        context.SetEmptyMass(LANDER_EMPTY_MASS);
        context.AddMesh("ShuttlePB");
    }
    fn pre_step(&mut self, _context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64)
    {
        // ffi::debugLog(& format!("{:?}", self.th_vernier));
        debugLog(& format!("Hello world! {} {} {}", sim_t, sim_dt, mjd));
    }
}

make_orbiter_vessel!(RustSpacecraft::new());
