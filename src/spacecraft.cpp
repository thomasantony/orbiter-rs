
#define STRICT
#define ORBITER_MODULE
#include "windows.h"
#include "orbitersdk.h"
#include "spacecraft.h"
#include "orbiter-rs/src/lib.rs.h"
#include <memory>
#include <cstring>

using std::unique_ptr;

void debugLog(rust::Str s)
{
    std::string _s(s.data(), s.length());
    sprintf(oapiDebugString(), _s.c_str());
}

VesselContext::VesselContext(OBJHANDLE hVessel, int flightmodel)
    : VESSEL4(hVessel, flightmodel),
      // rust_spacecraft_(box_to_uptr(create_rust_spacecraft()))
      rust_spacecraft_(std::move(create_rust_spacecraft()))
{
}

VesselContext::~VesselContext()
{
}

// Rust shims
void VesselContext::AddMesh(rust::Str mesh_name) const
{
    const std::string _mesh_name(mesh_name);
    VESSEL4::AddMesh(_mesh_name.data());
}
void VesselContext::AddMeshWithOffset(rust::Str mesh_name, const Vector3& ofs) const
{
    const std::string _mesh_name(mesh_name);
    VESSEL4::AddMesh(_mesh_name.data(), &ofs);
}
size_t VesselContext::AddExhaust(uintptr_t th, double lscale, double wscale) const
{
    return VESSEL4::AddExhaust(THRUSTER_HANDLE(th), lscale, wscale);
}
void VesselContext::SetPMI(const Vector3& pmi) const
{
    VESSEL4::SetPMI(pmi);
}
void VesselContext::SetCameraOffset(const Vector3 &co) const
{
    VESSEL4::SetCameraOffset(co);
}
void VesselContext::SetTouchdownPoints(const Vector3 &pt1, const Vector3 &pt2, const Vector3 &pt3) const
{
    VESSEL4::SetTouchdownPoints(pt1, pt2, pt3);
}

uintptr_t VesselContext::CreateThruster(const Vector3 &pos, const Vector3 &dir, double maxth0, uintptr_t ph, double isp) const
{
    return reinterpret_cast<uintptr_t>(VESSEL4::CreateThruster(pos, dir, maxth0, PROPELLANT_HANDLE(ph), isp));
}
uintptr_t VesselContext::CreatePropellantResource(double mass) const
{
    return reinterpret_cast<uintptr_t>(VESSEL4::CreatePropellantResource(mass));
}
uintptr_t VesselContext::CreateThrusterGroup(rust::Slice<const uintptr_t> thrusters, THGROUP_TYPE thgroup_type) const
{
    const uintptr_t *th_ptr = thrusters.data();

    return reinterpret_cast<uintptr_t>(VESSEL4::CreateThrusterGroup((THRUSTER_HANDLE *)th_ptr, thrusters.size(), thgroup_type));
}
double VesselContext::GetPropellantMass(uintptr_t ph) const
{
    return VESSEL4::GetPropellantMass(PROPELLANT_HANDLE(ph));
}
double VesselContext::GetThrusterGroupLevelByType(THGROUP_TYPE thgroup_type) const
{
    return VESSEL4::GetThrusterGroupLevel(thgroup_type);
}
rust::Str VesselContext::GetName() const
{
    return rust::Str(VESSEL4::GetName());
}
void VesselContext::SetThrusterDir(uintptr_t th, const Vector3 &dir) const
{
    VESSEL4::SetThrusterDir(THRUSTER_HANDLE(th), dir);
}
void VesselContext::SetThrusterLevel(uintptr_t th, double level) const
{
    VESSEL4::SetThrusterLevel(THRUSTER_HANDLE(th), level);
}

void VesselContext::clbkSetClassCaps(FILEHANDLE cfg)
{
    // physical vessel parameters
    // SetPMI(_V(0.50, 0.50, 0.50));
    dyn_vessel_set_class_caps(rust_spacecraft_, *this);
}

// Pre-step logic for differential thrust
void VesselContext::clbkPreStep(double SimT, double SimDT, double MJD)
{
    dyn_vessel_pre_step(rust_spacecraft_, *this, SimT, SimDT, MJD);
}








BoxDynVessel::BoxDynVessel(BoxDynVessel &&other) noexcept : repr(other.repr)
{
    other.repr = {0, 0};
}

BoxDynVessel::~BoxDynVessel() noexcept
{
    if (repr != std::array<std::uintptr_t, 2>{0, 0})
    {
        dyn_vessel_drop_in_place(this);
    }
}

// ==============================================================
// API callback interface
// ==============================================================

// --------------------------------------------------------------
// Vessel initialisation
// --------------------------------------------------------------
DLLCLBK VESSEL *ovcInit(OBJHANDLE hvessel, int flightmodel)
{
    return new VesselContext(hvessel, flightmodel);
}

// --------------------------------------------------------------
// Vessel cleanup
// --------------------------------------------------------------
DLLCLBK void ovcExit(VESSEL *vessel)
{
    if (vessel)
        delete (VesselContext *)vessel;
}
