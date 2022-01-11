/// Surveyor spacecraft definition using the SDK
use orbiter_rs::{
    ODebug, consts, oapi_create_vessel, OrbiterVessel, init_vessel,
    PropellantHandle, ThrusterHandle, Vector3, VesselContext, VesselStatus, THGROUP_TYPE, _V, DWORD,
};

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
const LEG_Z: f64 = -0.6;

#[derive(Debug, PartialEq)]
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
pub struct Surveyor {
    th_vernier: Vec<ThrusterHandle>,
    th_rcs: Vec<ThrusterHandle>,
    th_retro: ThrusterHandle,
    ph_vernier: PropellantHandle,
    ph_retro: PropellantHandle,
    ph_rcs: PropellantHandle,
    vehicle_state: SurveyorState,
}
impl Surveyor {
    fn setup_meshes(&mut self, context: &VesselContext) {
        context.ClearMeshes();
        let mut meshes = Vec::new();
        meshes.push(("Surveyor-AMR", Vector3::new(0., 0., -0.6)));
        meshes.push(("Surveyor-Retro", Vector3::new(0., 0., -0.5)));
        meshes.push(("Surveyor-Lander", Vector3::new(0., 0.3, 0.)));

        let meshes_used = match self.vehicle_state {
            SurveyorState::BeforeRetroIgnition => &meshes[0..],
            SurveyorState::RetroFiring => &meshes[1..],
            SurveyorState::AfterRetro => &meshes[2..],
        };
        for (mesh, ofs) in meshes_used {
            context.AddMeshWithOffset(mesh.to_string(), &ofs);
        }
    }
    fn calc_empty_mass(&self, context: &VesselContext) -> f64 {
        let mut empty_mass = 0.0;
        // Jettison AMR when retro starts firing
        if context.GetPropellantMass(self.ph_retro) > 0.999 * RETRO_PROP_MASS {
            empty_mass += AMR_MASS;
        }
        // Add in retro mass while there is still retro fuel left
        if context.GetPropellantMass(self.ph_retro) > 1. {
            empty_mass += RETRO_EMPTY_MASS;
        }
        empty_mass += LANDER_EMPTY_MASS;
        return empty_mass;
    }
    fn spawn_object(&self, context: &VesselContext, classname: &str, ext: &str, offset: &Vector3) {
        let mut vs = VesselStatus::default();

        context.GetStatus(&mut vs);
        context.Local2Rel(offset, &mut vs.rpos);

        vs.eng_main = 0.0;
        vs.eng_hovr = 0.0;
        vs.status = 0;
        let new_object_name = format!("{}{}", context.GetName(), ext);

        oapi_create_vessel(new_object_name, classname.to_owned(), &vs);
    }
    fn jettison(&mut self, context: &VesselContext) {
        use SurveyorState::*;
        match self.vehicle_state {
            BeforeRetroIgnition => {
                self.vehicle_state = RetroFiring;
                self.spawn_object(context, "Surveyor_AMR", "-AMR", _V!(0., 0., -0.6));
            }
            RetroFiring => {
                self.vehicle_state = AfterRetro;
                self.spawn_object(context, "Surveyor_Retro", "-Retro", _V!(0., 0., -0.5));
            }
            _ => {}
        }
        self.setup_meshes(context);
    }
}
impl OrbiterVessel for Surveyor {
    fn set_class_caps(&mut self, context: &VesselContext) {
        context.SetSize(1.0);
        context.SetPMI(_V!(0.50, 0.50, 0.50));
        context.SetTouchdownPoints(
            _V!(0.0, LEG_RAD, LEG_Z),
            _V!(
                (60.0f64).to_radians().sin() * LEG_RAD,
                -0.5 * LEG_RAD,
                LEG_Z
            ),
            _V!(
                -(60.0f64).to_radians().sin() * LEG_RAD,
                -0.5 * LEG_RAD,
                LEG_Z
            ),
        );
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
        self.th_rcs.push(context.CreateThruster(
            _V!(-RCS_SPACE, RCS_RAD, RCS_Z),
            _V!(1.0, 0.0, 0.0),
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));
        self.th_rcs.push(context.CreateThruster(
            _V!(RCS_SPACE, RCS_RAD, RCS_Z),
            _V!(-1.0, 0.0, 0.0),
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));

        // Leg2 jets
        self.th_rcs.push(context.CreateThruster(
            _V!(
                (60.0f64).to_radians().sin() * RCS_RAD,
                -0.5 * RCS_RAD,
                RCS_Z - RCS_SPACE
            ),
            _V!(0., 0., 1.),
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));
        self.th_rcs.push(context.CreateThruster(
            _V!(
                (60.0f64).to_radians().sin() * RCS_RAD,
                -0.5 * RCS_RAD,
                RCS_Z + RCS_SPACE
            ),
            _V!(0., 0., -1.),
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));

        // Leg3 jets
        self.th_rcs.push(context.CreateThruster(
            _V!(
                -(60.0f64).to_radians().sin() * RCS_RAD,
                -0.5 * RCS_RAD,
                RCS_Z - RCS_SPACE
            ),
            _V!(0., 0., 1.),
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));
        self.th_rcs.push(context.CreateThruster(
            _V!(
                -(60.0f64).to_radians().sin() * RCS_RAD,
                -0.5 * RCS_RAD,
                RCS_Z + RCS_SPACE
            ),
            _V!(0., 0., -1.),
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));

        // Create RCS thruster groups
        let mut th_group = [ThrusterHandle::default(), ThrusterHandle::default()];

        th_group[0] = self.th_rcs[3]; // -Z #1
        th_group[1] = self.th_rcs[5]; // -Z #2
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttPitchdown);

        th_group[0] = self.th_rcs[2]; // +Z #1
        th_group[1] = self.th_rcs[4]; // +Z #2
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttPitchup);

        th_group[0] = self.th_rcs[0]; // +X
        context.CreateThrusterGroup(&th_group[..1], THGROUP_TYPE::AttBankright);

        th_group[0] = self.th_rcs[1]; // -X
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttBankleft);

        th_group[0] = self.th_rcs[3]; // -Z #1
        th_group[1] = self.th_rcs[4]; // +Z #2
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttYawright);

        th_group[0] = self.th_rcs[2]; // +Z #1
        th_group[1] = self.th_rcs[5]; // -Z #2
        context.CreateThrusterGroup(&th_group, THGROUP_TYPE::AttYawleft);

        for th in self.th_rcs.iter() {
            context.AddExhaust(*th, 0.1, 0.05);
        }

        self.th_retro = context.CreateThruster(
            _V!(0.0, 0.0, RETRO_STA),
            _V!(0.0, 0.0, 1.0),
            RETRO_THRUST,
            self.ph_retro,
            RETRO_ISP,
        );
        context.AddExhaust(self.th_retro, 2.0, 0.3);

        context.SetEmptyMass(LANDER_EMPTY_MASS);

        // camera parameters
        context.SetCameraOffset(_V!(0.0, 0.8, 0.0));
        self.setup_meshes(context)
    }
    fn pre_step(&mut self, context: &VesselContext, _sim_t: f64, _sim_dt: f64, _mjd: f64) {
        context.SetEmptyMass(self.calc_empty_mass(context));

        let pitch = context.GetThrusterGroupLevelByType(THGROUP_TYPE::AttPitchup)
            - context.GetThrusterGroupLevelByType(THGROUP_TYPE::AttPitchdown);
        let yaw = context.GetThrusterGroupLevelByType(THGROUP_TYPE::AttYawright)
            - context.GetThrusterGroupLevelByType(THGROUP_TYPE::AttYawleft);
        let roll = context.GetThrusterGroupLevelByType(THGROUP_TYPE::AttBankright)
            - context.GetThrusterGroupLevelByType(THGROUP_TYPE::AttBankleft);

        // Differential thrusting for attitude control
        context.SetThrusterDir(
            self.th_vernier[0],
            _V!(5.0f64.to_radians().sin() * roll, 0.0, 1.0),
        ); // Roll using the 5 degree offset
        context.SetThrusterDir(
            self.th_vernier[1],
            _V!(0.0, 0.0, 1.0 + 0.05 * (pitch - yaw)),
        );
        context.SetThrusterDir(
            self.th_vernier[2],
            _V!(0.0, 0.0, 1.0 + 0.05 * (pitch + yaw)),
        );

        if self.vehicle_state == SurveyorState::RetroFiring
            && context.GetPropellantMass(self.ph_retro) < 1.0
        {
            //Jettison the spent main retro
            self.jettison(context);
        }
        if self.vehicle_state == SurveyorState::BeforeRetroIgnition
            && context.GetPropellantMass(self.ph_retro) < 0.999 * RETRO_PROP_MASS
        {
            //Jettison the AMR if the retro has started burning
            self.jettison(context);
            //Relight the retro if needed
            context.SetThrusterLevel(self.th_retro, 1.0);
        }
        ODebug(&format!("Pitch: {}, Yaw: {}, Roll: {}", pitch, yaw, roll));
    }
    fn consume_buffered_key(
        &mut self,
        context: &VesselContext,
        key: DWORD,
        down: bool,
        kstate: [u8; consts::LKEY_COUNT],
    ) -> i32 {
        if !down {
            0
        } else if kstate[consts::OAPI_KEY_LSHIFT] & 0x80 == 1
            || kstate[consts::OAPI_KEY_RSHIFT] & 0x80 == 1
        {
            0
        } else {
            // unmodified keys
            match key.0 as usize {
                consts::OAPI_KEY_L => {
                    // Fire Retro
                    context.SetThrusterLevel(self.th_retro, 1.0);
                    1
                }
                _ => 0,
            }
        }
    }
}

init_vessel!(
    fn init(_h_vessel: OBJHANDLE, _flight_model: i32) {
        Surveyor::default()
    }
    fn exit() {}
);
