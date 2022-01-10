
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
size_t SpacecraftWrapper::AddExhaust(uintptr_t th, double lscale, double wscale) const
{
    return VESSEL4::AddExhaust(THRUSTER_HANDLE(th), lscale, wscale);
}
void SpacecraftWrapper::SetPMI(const Vector3& pmi) const
{
    VESSEL4::SetPMI(_V(pmi.x, pmi.y, pmi.z));
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
