
#define STRICT
#define ORBITER_MODULE
#include "windows.h"
#include "orbitersdk.h"
#include "vessel_context.h"
#include "orbiter-rs/src/ffi.rs.h"
#include <memory>
#include <cstring>
#include <array>

using std::unique_ptr;

void ODebug(rust::String s)
{
    std::string _s(s.data(), min(s.length(), 254));
    sprintf(oapiDebugString(), _s.c_str());
}
OBJHANDLE oapi_create_vessel(rust::String name, rust::String classname, const VESSELSTATUS &status)
{
    return oapiCreateVessel(name.c_str(), classname.c_str(), status);
}

// VesselContext::VesselContext(OBJHANDLE hVessel, int flightmodel, BoxDynVessel& box_vessel)
VesselContext::VesselContext(OBJHANDLE hVessel, int flightmodel, VesselInitFn fn)
    : VESSEL4(hVessel, flightmodel), rust_init_fn_(fn)
{
}

VesselContext::~VesselContext()
{
}

// Rust shims
void VesselContext::AddMesh(rust::String mesh_name) const
{
    VESSEL4::AddMesh(mesh_name.c_str());
}
void VesselContext::AddMeshWithOffset(rust::String mesh_name, const Vector3& ofs) const
{
    VESSEL4::AddMesh(mesh_name.c_str(), &ofs);
}
size_t VesselContext::AddExhaust(THRUSTER_HANDLE th, double lscale, double wscale) const
{
    return VESSEL4::AddExhaust(th, lscale, wscale);
}

THRUSTER_HANDLE VesselContext::CreateThruster(const Vector3 &pos, const Vector3 &dir, double maxth0, PROPELLANT_HANDLE ph, double isp) const
{
    return VESSEL4::CreateThruster(pos, dir, maxth0, PROPELLANT_HANDLE(ph), isp);
}
PROPELLANT_HANDLE VesselContext::CreatePropellantResource(double mass) const
{
    return VESSEL4::CreatePropellantResource(mass);
}
THGROUP_HANDLE VesselContext::CreateThrusterGroup(rust::Slice<const THRUSTER_HANDLE> thrusters, THGROUP_TYPE thgroup_type) const
{
    return VESSEL4::CreateThrusterGroup((THRUSTER_HANDLE*)thrusters.data(), thrusters.size(), thgroup_type);
}

rust::Str VesselContext::GetName() const
{
    return rust::Str(VESSEL4::GetName());
}
OBJHANDLE VesselContext::GetSurfaceRef() const
{
    return VESSEL4::GetSurfaceRef();
}
void VesselContext::clbkSetClassCaps(FILEHANDLE cfg)
{
    rust_spacecraft_ = std::move(rust_init_fn_(*this));
    dyn_vessel_set_class_caps(rust_spacecraft_, *this, cfg);
}

// ==============================================================
// VESSEL callback interface
// ==============================================================
// Pre-step callback
void VesselContext::clbkPreStep(double SimT, double SimDT, double MJD)
{
    dyn_vessel_pre_step(rust_spacecraft_, *this, SimT, SimDT, MJD);
}
void VesselContext::clbkPostStep(double SimT, double SimDT, double MJD)
{
    dyn_vessel_post_step(rust_spacecraft_, *this, SimT, SimDT, MJD);
}
int VesselContext::clbkConsumeBufferedKey(DWORD key, bool down, char *kstate)
{
    return dyn_vessel_consume_buffered_key(rust_spacecraft_, *this, key, down, kstate);
}

// ==============================================================
// API callback interface used by the init_vessel! macro
// ==============================================================
// --------------------------------------------------------------
// Vessel initialisation
// --------------------------------------------------------------
VESSEL *vessel_ovcInit(OBJHANDLE hvessel, int flightmodel, VesselInitFn fn)
{
    return new VesselContext(hvessel, flightmodel, fn);
}

// VESSEL *vessel_ovcInit(OBJHANDLE hvessel, int flightmodel, BoxDynVessel box_vessel)
// {
//     return new VesselContext(hvessel, flightmodel, box_vessel);
// }

// --------------------------------------------------------------
// Vessel cleanup
// --------------------------------------------------------------
void vessel_ovcExit(VESSEL *vessel)
{
    if (vessel)
        delete (VesselContext *)vessel;
}
