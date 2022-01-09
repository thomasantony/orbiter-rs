// ==============================================================
//                 ORBITER MODULE: Surveyor
//       Modified from ShuttlePB example from ORBITER SDK
//
// Surveyor.cpp
// Control module for Surveyor vessel class
//
// Notes:
// This is an example for a "minimal" vessel implementation which
// only overloads the clbkSetClassCaps method to define vessel
// capabilities and otherwise uses the default VESSEL class
// behaviour.
// ==============================================================

#define STRICT
#define ORBITER_MODULE

#include "orbitersdk.h"

// ==============================================================
// Some vessel parameters
// ==============================================================
const double VERNIER_PROP_MASS = 70.98;
const double VERNIER_ISP = 3200;
const double VERNIER_THRUST = 463;
const double VERNIER_RAD = 0.86;
const double VERNIER_STA = -0.5;

const double RCS_PROP_MASS = 2;
const double RCS_ISP = 630.0;
const double RCS_THRUST = 0.25;
const double RCS_RAD = 1;
const double RCS_STA = -0.5;
const double RCS_SPACE = 0.1;

const double RETRO_PROP_MASS = 560.64;
const double RETRO_THRUST = 39140;
const double RETRO_BURNTIME = 40.5;
const double RETRO_ITOT = RETRO_THRUST * RETRO_BURNTIME;
const double RETRO_ISP = RETRO_ITOT / RETRO_PROP_MASS;
const double RETRO_STA = -0.75;

const double LANDER_EMPTY_MASS = 289.10; //Basic bus plus payload minus AMR minus retro case
const double RETRO_EMPTY_MASS = 64.88;
const double AMR_MASS = 3.82;

const double LEG_RAD = 1.5;
const double LEG_STA = -0.6;

enum VehicleState {
	AMR_AND_RETRO_ATTACHED = 0,
	RETRO_ATTACHED = 1,
	LANDER_ONLY = 2,
};
// ==============================================================
// Shuttle-PB class interface
// ==============================================================

class Surveyor: public VESSEL3 {
public:
	Surveyor (OBJHANDLE hVessel, int flightmodel);
	~Surveyor ();
	void clbkSetClassCaps (FILEHANDLE cfg);
	void clbkPreStep(double SimT, double SimDT, double MJD);
	
	double CalcEmptyMass();
	int clbkConsumeBufferedKey(DWORD key, bool down, char* kstate);

	void SetupMeshes();
	void AddLanderMesh();
	void AddRetroMesh();
	void AddAMRMesh();

	void SpawnObject(char* classname, char* ext, VECTOR3 ofs);
	void Jettison();
private:
	THRUSTER_HANDLE th_vernier[3], th_retro, th_rcs[6], th_group[2];
	PROPELLANT_HANDLE ph_vernier, ph_rcs, ph_retro;

	VehicleState status;
};

Surveyor::Surveyor (OBJHANDLE hVessel, int flightmodel)
: VESSEL3 (hVessel, flightmodel)
{
}

Surveyor::~Surveyor ()
{
}

void Surveyor::AddLanderMesh() {
	VECTOR3 ofs = _V(0, 0.3, 0);
	AddMesh("Surveyor-Lander", &ofs);
}
void Surveyor::AddRetroMesh() {
	VECTOR3 ofs = _V(0, 0, -0.5);
	AddMesh("Surveyor-Retro", &ofs);
}
void Surveyor::AddAMRMesh() {
	VECTOR3 ofs = _V(0, 0, -0.6);
	AddMesh("Surveyor-AMR", &ofs);
}

void Surveyor::SetupMeshes() {
	ClearMeshes();
	switch (status) {
	case AMR_AND_RETRO_ATTACHED:
		AddAMRMesh();
	case RETRO_ATTACHED:
		AddRetroMesh();
	case LANDER_ONLY:
		AddLanderMesh();
	}
}
// ==============================================================
// Overloaded callback functions
// ==============================================================

// --------------------------------------------------------------
// Set the capabilities of the vessel class
// --------------------------------------------------------------
void Surveyor::clbkSetClassCaps (FILEHANDLE cfg)
{
	status = AMR_AND_RETRO_ATTACHED;
	// physical vessel parameters
	SetSize(1.0);
	SetPMI(_V(0.50, 0.50, 0.50));

	SetTouchdownPoints(_V(0, LEG_RAD, LEG_STA), _V(sqrt(3.0) / 2 * LEG_RAD, -0.5 * LEG_RAD, LEG_STA), _V(-sqrt(3.0) / 2 * LEG_RAD, -0.5 * LEG_RAD, LEG_STA));

	// propellant resources
	ph_vernier = CreatePropellantResource (VERNIER_PROP_MASS);
	ph_rcs = CreatePropellantResource(RCS_PROP_MASS);
	ph_retro = CreatePropellantResource(RETRO_PROP_MASS);

	// main engine
	th_vernier[0] = CreateThruster(_V(           0.0 * VERNIER_RAD,  1.0 * VERNIER_RAD, VERNIER_STA), _V(0, 0, 1), VERNIER_THRUST, ph_vernier, VERNIER_ISP);
	th_vernier[1] = CreateThruster(_V( sqrt(3.0) / 2 * VERNIER_RAD, -0.5 * VERNIER_RAD, VERNIER_STA), _V(0, 0, 1), VERNIER_THRUST, ph_vernier, VERNIER_ISP);
	th_vernier[2] = CreateThruster(_V(-sqrt(3.0) / 2 * VERNIER_RAD, -0.5 * VERNIER_RAD, VERNIER_STA), _V(0, 0, 1), VERNIER_THRUST, ph_vernier, VERNIER_ISP);
	CreateThrusterGroup(th_vernier, 3, THGROUP_MAIN);
	for (int i = 0; i < 3; i++) {
		AddExhaust(th_vernier[i], 1, 0.1);
	}

	//Roll (Leg1) jets
	th_rcs[0] = CreateThruster(_V(-RCS_SPACE, RCS_RAD, RCS_STA), _V(1, 0, 0), RCS_THRUST, ph_rcs, RCS_ISP);
	th_rcs[1] = CreateThruster(_V(RCS_SPACE, RCS_RAD, RCS_STA), _V(-1, 0, 0), RCS_THRUST, ph_rcs, RCS_ISP);

	//Leg2 jets
	th_rcs[2] = CreateThruster(_V(sqrt(3.0) / 2 * RCS_RAD, -0.5 * RCS_RAD, RCS_STA - RCS_SPACE), _V(0, 0, 1), RCS_THRUST, ph_rcs, RCS_ISP);
	th_rcs[3] = CreateThruster(_V(sqrt(3.0) / 2 * RCS_RAD, -0.5 * RCS_RAD, RCS_STA + RCS_SPACE), _V(0, 0, -1), RCS_THRUST, ph_rcs, RCS_ISP);

	//Leg3 jets
	th_rcs[4] = CreateThruster(_V(-sqrt(3.0) / 2 * RCS_RAD, -0.5 * RCS_RAD, RCS_STA - RCS_SPACE), _V(0, 0, 1), RCS_THRUST, ph_rcs, RCS_ISP);
	th_rcs[5] = CreateThruster(_V(-sqrt(3.0) / 2 * RCS_RAD, -0.5 * RCS_RAD, RCS_STA + RCS_SPACE), _V(0, 0, -1), RCS_THRUST, ph_rcs, RCS_ISP);

	th_group[0] = th_rcs[3];	// -Z #1
	th_group[1] = th_rcs[5];	// -Z #2
	CreateThrusterGroup(th_group, 2, THGROUP_ATT_PITCHDOWN);

	th_group[0] = th_rcs[2];	// +Z #1
	th_group[1] = th_rcs[4];	// +Z #2
	CreateThrusterGroup(th_group, 2, THGROUP_ATT_PITCHUP);

	th_group[0] = th_rcs[0];	// +X
	CreateThrusterGroup(th_group, 1, THGROUP_ATT_BANKRIGHT);

	th_group[0] = th_rcs[1];	// -X
	CreateThrusterGroup(th_group, 1, THGROUP_ATT_BANKLEFT);

	th_group[0] = th_rcs[3];	// -Z #1
	th_group[1] = th_rcs[4];	// +Z #2
	CreateThrusterGroup(th_group, 2, THGROUP_ATT_YAWRIGHT);

	th_group[0] = th_rcs[2];	// +Z #1
	th_group[1] = th_rcs[5];	// -Z #2
	CreateThrusterGroup(th_group, 2, THGROUP_ATT_YAWLEFT);

	for (int i = 0; i < 6; i++) {
		AddExhaust(th_rcs[i], 0.1, 0.05);
	}

	th_retro = CreateThruster(_V(0.0, 0.0, RETRO_STA), _V(0, 0, 1), RETRO_THRUST, ph_retro, RETRO_ISP);
	AddExhaust(th_retro, 2, 0.3);
	// camera parameters
	SetCameraOffset (_V(0,0.8,0));

	// associate a mesh for the visual
	SetupMeshes();
}

// ==============================================================
// API callback interface
// ==============================================================

// --------------------------------------------------------------
// Vessel initialisation
// --------------------------------------------------------------
DLLCLBK VESSEL *ovcInit (OBJHANDLE hvessel, int flightmodel)
{
	return new Surveyor (hvessel, flightmodel);
}

// --------------------------------------------------------------
// Vessel cleanup
// --------------------------------------------------------------
DLLCLBK void ovcExit (VESSEL *vessel)
{
	if (vessel) delete (Surveyor*)vessel;
}
// Pre-step logic for differential thrust
void Surveyor::clbkPreStep(double SimT, double SimDT, double MJD) {
	SetEmptyMass(CalcEmptyMass());

	double P, Y, R;
	P = GetThrusterGroupLevel(THGROUP_ATT_PITCHUP) - GetThrusterGroupLevel(THGROUP_ATT_PITCHDOWN);
	Y = GetThrusterGroupLevel(THGROUP_ATT_YAWRIGHT) - GetThrusterGroupLevel(THGROUP_ATT_YAWLEFT);
	R = GetThrusterGroupLevel(THGROUP_ATT_BANKRIGHT) - GetThrusterGroupLevel(THGROUP_ATT_BANKLEFT);
	sprintf(oapiDebugString(), "Pitch %f Yaw %f Roll %f", P, Y, R);

	SetThrusterDir(th_vernier[0], _V(0.087 * R, 0, 1));	// Roll using the 5 degree offset
	SetThrusterDir(th_vernier[1], _V(0, 0, 1.0 + 0.05 * (P - Y)));
	SetThrusterDir(th_vernier[2], _V(0, 0, 1.0 + 0.05 * (P + Y)));

	if (status == RETRO_ATTACHED && GetPropellantMass(ph_retro) < 1) {
		//Jettison the spent main retro
		Jettison();
	}
	if (status == AMR_AND_RETRO_ATTACHED && GetPropellantMass(ph_retro) < 0.999 * RETRO_PROP_MASS) {
		//Jettison the AMR if the retro has started burning
		Jettison();
		//Relight the retro if needed
		SetThrusterLevel(th_retro, 1);
	}
}

double Surveyor::CalcEmptyMass() {
	double EmptyMass = 0;
	// Jettison AMR when retro starts firing
	if (GetPropellantMass(ph_retro) > 0.999 * RETRO_PROP_MASS) {
		EmptyMass += AMR_MASS;
	}
	// Add in retro mass while there is still retro fuel left
	if (GetPropellantMass(ph_retro) > 1) {
		EmptyMass += RETRO_EMPTY_MASS;
	}
	EmptyMass += LANDER_EMPTY_MASS;
	return EmptyMass;
}

int Surveyor::clbkConsumeBufferedKey(DWORD key, bool down, char* kstate) {
	if (!down) return 0; // only process keydown events

	if (KEYMOD_SHIFT(kstate)) {

	}
	else { // unmodified keys
		switch (key) {
		case OAPI_KEY_L:  // Fire Retro
			SetThrusterLevel(th_retro, 1);
			return 1;
		}
	}
	return 0;
}

void Surveyor::SpawnObject(char* classname, char* ext, VECTOR3 ofs) {
	VESSELSTATUS vs;
	char name[256];
	GetStatus(vs);
	Local2Rel(ofs, vs.rpos);
	vs.eng_main = vs.eng_hovr = 0.0;
	vs.status = 0;
	strcpy(name, GetName()); strcat(name, ext);
	oapiCreateVessel(name, classname, vs);
}
void Surveyor::Jettison() {
	switch (status) {
	case AMR_AND_RETRO_ATTACHED:
		status = RETRO_ATTACHED;
		SpawnObject("Surveyor_AMR", "-AMR", _V(0, 0, -0.6));
		break;
	case RETRO_ATTACHED:
		status = LANDER_ONLY;
		SpawnObject("Surveyor_Retro", "-Retro", _V(0, 0, -0.5));
		break;
	}
	SetupMeshes();
}