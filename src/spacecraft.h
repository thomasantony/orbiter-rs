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
    void AddExhaust(rust::Str mesh_name) const;
    size_t AddExhaust(THRUSTER_HANDLE th, double lscale, double wscale) const;

    void SetPMI(const Vector3& pmi) const;
    void SetCameraOffset(const Vector3& co) const;
    void SetTouchdownPoints(const Vector3 &pt1, const Vector3 &pt2, const Vector3 &pt3) const;
    void SetThrusterDir(THRUSTER_HANDLE th, const Vector3 &dir) const;
    void SetThrusterLevel(THRUSTER_HANDLE th, double level) const;

    THRUSTER_HANDLE CreateThruster(const Vector3 &pos, const Vector3 &dir, double maxth0, PROPELLANT_HANDLE ph, double isp) const;
    PROPELLANT_HANDLE CreatePropellantResource(double mass) const;

    uintptr_t CreateThrusterGroup(rust::Slice<const THRUSTER_HANDLE> thrusters, THGROUP_TYPE thgroup_type) const;

    rust::Str GetName() const;
    double GetPropellantMass(PROPELLANT_HANDLE ph) const;
    double GetThrusterGroupLevelByType(THGROUP_TYPE thgroup_type) const;
    // int clbkConsumeBufferedKey(DWORD key, bool down, char *kstate);

    // void SetupMeshes();
    // void AddLanderMesh();
    // void AddRetroMesh();
    // void AddAMRMesh();

    // void SpawnObject(char *classname, char *ext, VECTOR3 ofs);
    // void Jettison();

private:
    BoxDynVessel rust_spacecraft_;
    // THRUSTER_HANDLE th_vernier[3], th_retro, th_rcs[6], th_group[2];
    // PROPELLANT_HANDLE ph_vernier, ph_rcs, ph_retro;
};
