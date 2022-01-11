#pragma once
// DO NOT IMPORT lib.rs.h here
#include "rust/cxx.h"
#include "orbitersdk.h"
#include <cstdint>
void debugLog(rust::Str);

using Vector3 = VECTOR3;

class BoxDynVessel
{
public:
    BoxDynVessel(BoxDynVessel &&) noexcept;
    ~BoxDynVessel() noexcept;
    using IsRelocatable = std::true_type;

private:
    std::array<std::uintptr_t, 2> repr;
};
using PtrBoxDynVessel = BoxDynVessel *;
// ==============================================================
// Spacecraft class interface
// ==============================================================
class VesselContext : public VESSEL4
{
public:
    VesselContext(OBJHANDLE hVessel, int flightmodel);
    ~VesselContext();
    void clbkSetClassCaps(FILEHANDLE cfg);
    void clbkPreStep(double SimT, double SimDT, double MJD);

    void AddMesh(rust::Str mesh_name) const;
    void AddMeshWithOffset(rust::Str mesh_name, const Vector3& ofs) const;
    size_t AddExhaust(THRUSTER_HANDLE th, double lscale, double wscale) const;

    THRUSTER_HANDLE CreateThruster(const Vector3 &pos, const Vector3 &dir, double maxth0, PROPELLANT_HANDLE ph, double isp) const;
    PROPELLANT_HANDLE CreatePropellantResource(double mass) const;
    THGROUP_HANDLE CreateThrusterGroup(rust::Slice<const THRUSTER_HANDLE> thrusters, THGROUP_TYPE thgroup_type) const;

    rust::Str GetName() const;
    // double GetPropellantMass(PROPELLANT_HANDLE ph) const;
    double GetThrusterGroupLevelByType(THGROUP_TYPE thgroup_type) const;
    // int clbkConsumeBufferedKey(DWORD key, bool down, char *kstate);

    // void SpawnObject(char *classname, char *ext, VECTOR3 ofs);
    // void Jettison();

private:
    BoxDynVessel rust_spacecraft_;
};
