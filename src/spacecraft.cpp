
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

SpacecraftWrapper::SpacecraftWrapper(OBJHANDLE hVessel, int flightmodel)
    : VESSEL4(hVessel, flightmodel),
      // rust_spacecraft_(box_to_uptr(create_rust_spacecraft()))
      rust_spacecraft_(std::move(create_rust_spacecraft()))
{
}

SpacecraftWrapper::~SpacecraftWrapper()
{
}

// Rust shims
void SpacecraftWrapper::AddMesh(rust::Str mesh_name) const
{
    const std::string _mesh_name(mesh_name);
    VESSEL4::AddMesh(_mesh_name.data());
}
void SpacecraftWrapper::AddMeshWithOffset(rust::Str mesh_name, const Vector3& ofs) const
{
    const std::string _mesh_name(mesh_name);
    auto _ofs = _V(ofs.x, ofs.y, ofs.z);
    VESSEL4::AddMesh(_mesh_name.data(), &_ofs);
}
size_t SpacecraftWrapper::AddExhaust(uintptr_t th, double lscale, double wscale) const
{
    return VESSEL4::AddExhaust(THRUSTER_HANDLE(th), lscale, wscale);
}
void SpacecraftWrapper::SetPMI(const Vector3& pmi) const
{
    VESSEL4::SetPMI(_V(pmi.x, pmi.y, pmi.z));
}
void SpacecraftWrapper::SetCameraOffset(const Vector3 &co) const
{
    VESSEL4::SetCameraOffset(_V(co.x, co.y, co.z));
}
void SpacecraftWrapper::SetTouchdownPoints(const Vector3 &pt1, const Vector3 &pt2, const Vector3 &pt3) const
{
    VESSEL4::SetTouchdownPoints(_V(pt1.x, pt1.y, pt1.z), _V(pt2.x, pt2.y, pt2.z), _V(pt3.x, pt3.y, pt3.z));
}

uintptr_t SpacecraftWrapper::CreateThruster(const Vector3 &pos, const Vector3 &dir, double maxth0, uintptr_t ph, double isp) const
{
    return reinterpret_cast<uintptr_t>(VESSEL4::CreateThruster(_V(pos.x, pos.y, pos.z), _V(dir.x, dir.y, dir.z), maxth0, PROPELLANT_HANDLE(ph), isp));
}
uintptr_t SpacecraftWrapper::CreatePropellantResource(double mass) const
{
    return reinterpret_cast<uintptr_t>(VESSEL4::CreatePropellantResource(mass));
}
uintptr_t SpacecraftWrapper::CreateThrusterGroup(rust::Slice<const uintptr_t> thrusters, THGROUP_TYPE thgroup_type) const
{
    const uintptr_t *th_ptr = thrusters.data();

    return reinterpret_cast<uintptr_t>(VESSEL4::CreateThrusterGroup((THRUSTER_HANDLE *)th_ptr, thrusters.size(), thgroup_type));
}
double SpacecraftWrapper::GetPropellantMass(uintptr_t ph) const
{
    return VESSEL4::GetPropellantMass(PROPELLANT_HANDLE(ph));
}
double SpacecraftWrapper::GetThrusterGroupLevelByType(THGROUP_TYPE thgroup_type) const
{
    return VESSEL4::GetThrusterGroupLevel(thgroup_type);
}
void SpacecraftWrapper::SetThrusterDir(uintptr_t th, const Vector3 &dir) const
{
    VESSEL4::SetThrusterDir(THRUSTER_HANDLE(th), _V(dir.x, dir.y, dir.z));
}
void SpacecraftWrapper::SetThrusterLevel(uintptr_t th, double level) const
{
    VESSEL4::SetThrusterLevel(THRUSTER_HANDLE(th), level);
}

void SpacecraftWrapper::clbkSetClassCaps(FILEHANDLE cfg)
{
    // physical vessel parameters
    // SetPMI(_V(0.50, 0.50, 0.50));
    dyn_vessel_set_class_caps(rust_spacecraft_, *this);
}

// Pre-step logic for differential thrust
void SpacecraftWrapper::clbkPreStep(double SimT, double SimDT, double MJD)
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
    return new SpacecraftWrapper(hvessel, flightmodel);
}

// --------------------------------------------------------------
// Vessel cleanup
// --------------------------------------------------------------
DLLCLBK void ovcExit(VESSEL *vessel)
{
    if (vessel)
        delete (SpacecraftWrapper *)vessel;
}
