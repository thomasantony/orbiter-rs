/// Surveyor spacecraft definition using the SDK
use crate::{debugLog, make_orbiter_vessel, OrbiterVessel, SpacecraftWrapper, THGROUP_TYPE, _V};

const VERNIER_PROP_MASS: f64 = 70.98;
const VERNIER_ISP: f64 = 3200.0;
const VERNIER_THRUST: f64 = 463.0;
const VERNIER_RAD: f64 = 0.86;
const VERNIER_Z: f64 = -0.5;

const RCS_PROP_MASS: f64 = 2.0;
const RCS_ISP: f64 = 630.0;
const RCS_THRUST: f64 = 0.25;
const RCS_RAD: f64 = 1.0;
const RCS_Z: f64 = -0.5;
const RCS_SPACE: f64 = 0.1;

const RETRO_PROP_MASS: f64 = 560.64;
const RETRO_THRUST: f64 = 39140.0;
const RETRO_BURNTIME: f64 = 40.5;
const RETRO_ITOT: f64 = RETRO_THRUST * RETRO_BURNTIME;
const RETRO_ISP: f64 = RETRO_ITOT / RETRO_PROP_MASS;
const RETRO_STA: f64 = -0.75;

const LANDER_EMPTY_MASS: f64 = 289.10; //Basic bus plus payload minus AMR minus retro case
const RETRO_EMPTY_MASS: f64 = 64.88;
const AMR_MASS: f64 = 3.82;

const LEG_RAD: f64 = 1.5;
const LEG_STA: f64 = -0.6;

#[derive(Debug)]
enum SurveyorState {
    BeforeRetroIgnition,
    RetroFiring,
    AfterRetro,
}
impl Default for SurveyorState {
    fn default() -> Self {
        Self::BeforeRetroIgnition
    }
}

#[derive(Default, Debug)]
pub struct RustSpacecraft {
    th_vernier: Vec<usize>,
    th_rcs: Vec<usize>,
    th_retro: usize,
    ph_vernier: usize,
    ph_retro: usize,
    ph_rcs: usize,
    vehicle_state: SurveyorState,
}
impl RustSpacecraft {
    fn setup_meshes(&mut self, context: &SpacecraftWrapper)
    {
        context.ClearMeshes();
        match self.vehicle_state {
            SurveyorState::BeforeRetroIgnition => context.AddMeshWithOffset("Surveyor-AMR", _V!(0., 0., -0.6)),
            SurveyorState::RetroFiring => context.AddMeshWithOffset("Surveyor-Retro", _V!(0., 0., -0.5)),
            SurveyorState::AfterRetro => context.AddMeshWithOffset("Surveyor-Lander", _V!(0., 0.3, 0.)),
        }
    }
}
impl OrbiterVessel for RustSpacecraft {
    fn set_class_caps(&mut self, context: &SpacecraftWrapper) {
        context.SetSize(1.0);
        context.SetPMI(_V!(0.50, 0.50, 0.50));

        // Create Propellant Resources
        self.ph_vernier = context.CreatePropellantResource(VERNIER_PROP_MASS);
	    self.ph_rcs = context.CreatePropellantResource(RCS_PROP_MASS);
	    self.ph_retro = context.CreatePropellantResource(RETRO_PROP_MASS);

        self.th_vernier.push(context.CreateThruster(
            _V!(0.0 * VERNIER_RAD, 1.0 * VERNIER_RAD, VERNIER_Z),
            _V!(0.0, 0.0, 1.0),
            VERNIER_THRUST,
            self.ph_vernier,
            VERNIER_ISP,
        ));
        self.th_vernier.push(context.CreateThruster(
            _V!(
                (60.0f64).to_radians().sin() * VERNIER_RAD,
                -0.5 * VERNIER_RAD,
                VERNIER_Z
            ),
            _V!(0.0, 0.0, 1.0),
            VERNIER_THRUST,
            self.ph_vernier,
            VERNIER_ISP,
        ));
        self.th_vernier.push(context.CreateThruster(
            _V!(
                (-60.0f64).to_radians().sin() * VERNIER_RAD,
                -0.5 * VERNIER_RAD,
                VERNIER_Z
            ),
            _V!(0.0, 0.0, 1.0),
            VERNIER_THRUST,
            self.ph_vernier,
            VERNIER_ISP,
        ));
        context.CreateThrusterGroup(&self.th_vernier, THGROUP_TYPE::Main);
        for th in self.th_vernier.iter() {
            context.AddExhaust(*th, 1.0, 0.1);
        }

        // Roll (Leg1) jets
        self.th_rcs.push(context.CreateThruster(_V!(-RCS_SPACE, RCS_RAD, RCS_Z), _V!(1.0, 0.0, 0.0), RCS_THRUST, self.ph_rcs, RCS_ISP));
        self.th_rcs.push(context.CreateThruster(_V!(RCS_SPACE, RCS_RAD, RCS_Z), _V!(-1.0, 0.0, 0.0), RCS_THRUST, self.ph_rcs, RCS_ISP));

        // Leg2 jets
        self.th_rcs.push(context.CreateThruster(_V!((60.0f64).to_radians().sin() * RCS_RAD, -0.5 * RCS_RAD, RCS_Z - RCS_SPACE), _V!(0., 0., 1.), RCS_THRUST, self.ph_rcs, RCS_ISP));
        self.th_rcs.push(context.CreateThruster(_V!((60.0f64).to_radians().sin() * RCS_RAD, -0.5 * RCS_RAD, RCS_Z + RCS_SPACE), _V!(0., 0., -1.), RCS_THRUST, self.ph_rcs, RCS_ISP));

        // Leg3 jets
        self.th_rcs.push(context.CreateThruster(_V!(-(60.0f64).to_radians().sin() * RCS_RAD, -0.5 * RCS_RAD, RCS_Z - RCS_SPACE), _V!(0., 0., 1.), RCS_THRUST, self.ph_rcs, RCS_ISP));
        self.th_rcs.push(context.CreateThruster(_V!(-(60.0f64).to_radians().sin() * RCS_RAD, -0.5 * RCS_RAD, RCS_Z + RCS_SPACE), _V!(0., 0., -1.), RCS_THRUST, self.ph_rcs, RCS_ISP));

        // Create RCS thruster groups
        let mut th_group: [usize; 2] = [0, 0];
        th_group[0] = self.th_rcs[3];	// -Z #1
        th_group[1] = self.th_rcs[5];	// -Z #2
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttPitchdown);

        th_group[0] = self.th_rcs[2];	// +Z #1
        th_group[1] = self.th_rcs[4];	// +Z #2
        context.CreateThrusterGroup(&th_group,THGROUP_TYPE::AttPitchup);

        th_group[0] = self.th_rcs[0];	// +X
        context.CreateThrusterGroup(&th_group[..1], THGROUP_TYPE::AttBankright);

        th_group[0] = self.th_rcs[1];	// -X
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttBankleft);

        th_group[0] = self.th_rcs[3];	// -Z #1
        th_group[1] = self.th_rcs[4];	// +Z #2
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttYawright);

        th_group[0] = self.th_rcs[2];	// +Z #1
        th_group[1] = self.th_rcs[5];	// -Z #2
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttYawleft);

        for th in self.th_rcs.iter() {
            context.AddExhaust(*th, 0.1, 0.05);
        }
        
        self.th_retro = context.CreateThruster(_V!(0.0, 0.0, RETRO_STA), _V!(0.0, 0.0, 1.0), RETRO_THRUST, self.ph_retro, RETRO_ISP);
	    context.AddExhaust(self.th_retro, 2.0, 0.3);

        context.SetEmptyMass(LANDER_EMPTY_MASS);
        
        // camera parameters
	    context.SetCameraOffset (_V!(0.0, 0.8, 0.0));
        self.setup_meshes(context)
    }
    fn pre_step(&mut self, _context: &SpacecraftWrapper, sim_t: f64, sim_dt: f64, mjd: f64) {
        // ffi::debugLog(& format!("{:?}", self.th_vernier));
        debugLog(&format!("Hello world! {} {} {}", sim_t, sim_dt, mjd));
    }
}

make_orbiter_vessel!(RustSpacecraft::default());
