use lazy_static::lazy_static;
/// Surveyor spacecraft implementation using orbiter-rs
///
/// This is a port of Surveyor.cpp to Rust
///
use orbiter_rs::{
    debug_string, init_vessel, oapi_create_vessel, FileHandle, Key, KeyStates, OrbiterVessel,
    PropellantHandle, SDKVessel, ThrusterGroupType, ThrusterHandle, Vector3, VesselStatus, V,
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
const RETRO_Z: f64 = -0.75;

const LANDER_EMPTY_MASS: f64 = 289.10; //Basic bus plus payload minus AMR minus retro case
const RETRO_EMPTY_MASS: f64 = 64.88;
const AMR_MASS: f64 = 3.82;

const LEG_RAD: f64 = 1.5;
const LEG_Z: f64 = -0.6;

lazy_static! {
    static ref THRUSTER1_POS: Vector3 =
        Vector3::new(0.0 * VERNIER_RAD, 1.0 * VERNIER_RAD, VERNIER_Z);
    static ref THRUSTER2_POS: Vector3 = Vector3::new(
        (60.0f64).to_radians().sin() * VERNIER_RAD,
        -0.5 * VERNIER_RAD,
        VERNIER_Z
    );
    static ref THRUSTER3_POS: Vector3 = Vector3::new(
        -(60.0f64).to_radians().sin() * VERNIER_RAD,
        -0.5 * VERNIER_RAD,
        VERNIER_Z
    );
    static ref DIR_X_PLUS: Vector3 = Vector3::new(1., 0., 0.);
    static ref DIR_X_MINUS: Vector3 = Vector3::new(-1., 0., 0.);
    static ref DIR_Y_PLUS: Vector3 = Vector3::new(0., 1., 0.);
    static ref DIR_Y_MINUS: Vector3 = Vector3::new(0., -1., 0.);
    static ref DIR_Z_PLUS: Vector3 = Vector3::new(0., 0., 1.);
    static ref DIR_Z_MINUS: Vector3 = Vector3::new(0., 0., 1.);
}

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

#[derive(Debug)]
pub struct Surveyor {
    ctx: SDKVessel,
    th_vernier: Vec<ThrusterHandle>,
    th_rcs: Vec<ThrusterHandle>,
    th_retro: ThrusterHandle,
    ph_vernier: PropellantHandle,
    ph_retro: PropellantHandle,
    ph_rcs: PropellantHandle,
    vehicle_state: SurveyorState,
}
impl Surveyor {
    pub fn new(vessel: SDKVessel) -> Self {
        Self {
            ctx: vessel,
            th_vernier: Vec::new(),
            th_rcs: Vec::new(),
            th_retro: ThrusterHandle::default(),
            ph_vernier: PropellantHandle::default(),
            ph_retro: PropellantHandle::default(),
            ph_rcs: PropellantHandle::default(),
            vehicle_state: SurveyorState::default(),
        }
    }
    fn setup_meshes(&mut self) {
        self.ctx.ClearMeshes();
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
            self.ctx.AddMeshWithOffset(mesh.to_string(), &ofs);
        }
    }
    fn calc_empty_mass(&self) -> f64 {
        let mut empty_mass = 0.0;
        // Jettison AMR when retro starts firing
        if self.ctx.GetPropellantMass(self.ph_retro) > 0.999 * RETRO_PROP_MASS {
            empty_mass += AMR_MASS;
        }
        // Add in retro mass while there is still retro fuel left
        if self.ctx.GetPropellantMass(self.ph_retro) > 1. {
            empty_mass += RETRO_EMPTY_MASS;
        }
        empty_mass += LANDER_EMPTY_MASS;
        return empty_mass;
    }
    fn spawn_object(&self, classname: &str, ext: &str, offset: &Vector3) {
        let mut vs = VesselStatus::default();

        self.ctx.GetStatus(&mut vs);
        self.ctx.Local2Rel(offset, &mut vs.rpos);

        vs.eng_main = 0.0;
        vs.eng_hovr = 0.0;
        vs.status = 0;
        let new_object_name = format!("{}{}", self.ctx.GetName(), ext);

        oapi_create_vessel(new_object_name, classname.to_owned(), &vs);
    }
    fn jettison(&mut self) {
        use SurveyorState::*;
        match self.vehicle_state {
            BeforeRetroIgnition => {
                self.vehicle_state = RetroFiring;
                self.spawn_object("Surveyor_AMR", "-AMR", &V!(0., 0., -0.6));
            }
            RetroFiring => {
                self.vehicle_state = AfterRetro;
                self.spawn_object("Surveyor_Retro", "-Retro", &V!(0., 0., -0.5));
            }
            _ => {}
        }
        self.setup_meshes();
    }
}
impl OrbiterVessel for Surveyor {
    fn set_class_caps(&mut self, _cfg: &FileHandle) {
        self.ctx.SetSize(1.0);
        self.ctx.SetPMI(&V!(0.50, 0.50, 0.50));
        self.ctx.SetTouchdownPoints(
            &V!(0.0, LEG_RAD, LEG_Z),
            &V!(
                (60.0f64).to_radians().sin() * LEG_RAD,
                -0.5 * LEG_RAD,
                LEG_Z
            ),
            &V!(
                -(60.0f64).to_radians().sin() * LEG_RAD,
                -0.5 * LEG_RAD,
                LEG_Z
            ),
        );
        // Create Propellant Resources
        self.ph_vernier = self.ctx.CreatePropellantResource(VERNIER_PROP_MASS);
        self.ph_rcs = self.ctx.CreatePropellantResource(RCS_PROP_MASS);
        self.ph_retro = self.ctx.CreatePropellantResource(RETRO_PROP_MASS);

        self.th_vernier.push(self.ctx.CreateThruster(
            &THRUSTER1_POS,
            &DIR_Z_PLUS,
            VERNIER_THRUST,
            self.ph_vernier,
            VERNIER_ISP,
        ));
        self.th_vernier.push(self.ctx.CreateThruster(
            &THRUSTER2_POS,
            &DIR_Z_PLUS,
            VERNIER_THRUST,
            self.ph_vernier,
            VERNIER_ISP,
        ));
        self.th_vernier.push(self.ctx.CreateThruster(
            &THRUSTER3_POS,
            &DIR_Z_PLUS,
            VERNIER_THRUST,
            self.ph_vernier,
            VERNIER_ISP,
        ));
        self.ctx
            .CreateThrusterGroup(&self.th_vernier, ThrusterGroupType::Main);
        for th in self.th_vernier.iter() {
            self.ctx.AddExhaust(*th, 1.0, 0.1);
        }

        // Roll (Leg1) jets
        self.th_rcs.push(self.ctx.CreateThruster(
            &V!(-RCS_SPACE, RCS_RAD, RCS_Z),
            &DIR_X_PLUS,
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));
        self.th_rcs.push(self.ctx.CreateThruster(
            &V!(RCS_SPACE, RCS_RAD, RCS_Z),
            &DIR_X_MINUS,
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));

        // Leg2 jets
        self.th_rcs.push(self.ctx.CreateThruster(
            &V!(
                (60.0f64).to_radians().sin() * RCS_RAD,
                -0.5 * RCS_RAD,
                RCS_Z - RCS_SPACE
            ),
            &DIR_Z_PLUS,
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));
        self.th_rcs.push(self.ctx.CreateThruster(
            &V!(
                (60.0f64).to_radians().sin() * RCS_RAD,
                -0.5 * RCS_RAD,
                RCS_Z + RCS_SPACE
            ),
            &DIR_Z_MINUS,
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));

        // Leg3 jets
        self.th_rcs.push(self.ctx.CreateThruster(
            &V!(
                -(60.0f64).to_radians().sin() * RCS_RAD,
                -0.5 * RCS_RAD,
                RCS_Z - RCS_SPACE
            ),
            &DIR_Z_PLUS,
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));
        self.th_rcs.push(self.ctx.CreateThruster(
            &V!(
                -(60.0f64).to_radians().sin() * RCS_RAD,
                -0.5 * RCS_RAD,
                RCS_Z + RCS_SPACE
            ),
            &DIR_Z_MINUS,
            RCS_THRUST,
            self.ph_rcs,
            RCS_ISP,
        ));

        // Create RCS thruster groups
        let mut th_group = [ThrusterHandle::default(), ThrusterHandle::default()];

        th_group[0] = self.th_rcs[3]; // -Z #1
        th_group[1] = self.th_rcs[5]; // -Z #2
        self.ctx
            .CreateThrusterGroup(&th_group, ThrusterGroupType::AttPitchdown);

        th_group[0] = self.th_rcs[2]; // +Z #1
        th_group[1] = self.th_rcs[4]; // +Z #2
        self.ctx
            .CreateThrusterGroup(&th_group, ThrusterGroupType::AttPitchup);

        th_group[0] = self.th_rcs[0]; // +X
        self.ctx
            .CreateThrusterGroup(&th_group[..1], ThrusterGroupType::AttBankright);

        th_group[0] = self.th_rcs[1]; // -X
        self.ctx
            .CreateThrusterGroup(&th_group, ThrusterGroupType::AttBankleft);

        th_group[0] = self.th_rcs[3]; // -Z #1
        th_group[1] = self.th_rcs[4]; // +Z #2
        self.ctx
            .CreateThrusterGroup(&th_group, ThrusterGroupType::AttYawright);

        th_group[0] = self.th_rcs[2]; // +Z #1
        th_group[1] = self.th_rcs[5]; // -Z #2
        self.ctx
            .CreateThrusterGroup(&th_group, ThrusterGroupType::AttYawleft);

        for th in self.th_rcs.iter() {
            self.ctx.AddExhaust(*th, 0.1, 0.05);
        }

        self.th_retro = self.ctx.CreateThruster(
            &V!(0.0, 0.0, RETRO_Z),
            &DIR_Z_PLUS,
            RETRO_THRUST,
            self.ph_retro,
            RETRO_ISP,
        );
        self.ctx.AddExhaust(self.th_retro, 2.0, 0.3);

        self.ctx.SetEmptyMass(LANDER_EMPTY_MASS);

        // camera parameters
        self.ctx.SetCameraOffset(&V!(0.0, 0.8, 0.0));
        self.setup_meshes()
    }
    fn on_pre_step(&mut self, _sim_t: f64, _sim_dt: f64, _mjd: f64) {
        self.ctx.SetEmptyMass(self.calc_empty_mass());

        let pitch = self
            .ctx
            .GetThrusterGroupLevelByType(ThrusterGroupType::AttPitchup)
            - self
                .ctx
                .GetThrusterGroupLevelByType(ThrusterGroupType::AttPitchdown);
        let yaw = self
            .ctx
            .GetThrusterGroupLevelByType(ThrusterGroupType::AttYawright)
            - self
                .ctx
                .GetThrusterGroupLevelByType(ThrusterGroupType::AttYawleft);
        let roll = self
            .ctx
            .GetThrusterGroupLevelByType(ThrusterGroupType::AttBankright)
            - self
                .ctx
                .GetThrusterGroupLevelByType(ThrusterGroupType::AttBankleft);

        // Differential thrusting for attitude control
        self.ctx.SetThrusterDir(
            self.th_vernier[0],
            &V!(5.0f64.to_radians().sin() * roll, 0.0, 1.0),
        ); // Roll using the 5 degree offset
        self.ctx.SetThrusterDir(
            self.th_vernier[1],
            &V!(0.0, 0.0, 1.0 + 0.05 * (pitch - yaw)),
        );
        self.ctx.SetThrusterDir(
            self.th_vernier[2],
            &V!(0.0, 0.0, 1.0 + 0.05 * (pitch + yaw)),
        );

        if self.vehicle_state == SurveyorState::RetroFiring
            && self.ctx.GetPropellantMass(self.ph_retro) < 1.0
        {
            //Jettison the spent main retro
            self.jettison();
        }
        if self.vehicle_state == SurveyorState::BeforeRetroIgnition
            && self.ctx.GetPropellantMass(self.ph_retro) < 0.999 * RETRO_PROP_MASS
        {
            //Jettison the AMR if the retro has started burning
            self.jettison();
            //Relight the retro if needed
            self.ctx.SetThrusterLevel(self.th_retro, 1.0);
        }
        debug_string!("Pitch: {}, Yaw: {}, Roll: {}", pitch, yaw, roll);
    }
    fn consume_buffered_key(&mut self, key: Key, down: bool, kstate: KeyStates) -> i32 {
        if !down {
            0
        } else if kstate.shift() {
            0
        } else {
            // unmodified keys
            if key == Key::L {
                // Fire Retro
                self.ctx.SetThrusterLevel(self.th_retro, 1.0);
                1
            } else {
                1
            }
        }
    }
}

init_vessel!(
    fn init(vessel)
    {
        Surveyor::new(vessel)
    }
    fn exit() {}
);
