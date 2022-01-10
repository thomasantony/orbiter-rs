#pragma once
// DO NOT IMPORT lib.rs.h here
#include "rust/cxx.h"
#include "orbitersdk.h"
#include <cstdint>
void debugLog(rust::Str);

struct Vector3;
using c_void = void;


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
class SpacecraftWrapper : public VESSEL4
{
public:
    SpacecraftWrapper(OBJHANDLE hVessel, int flightmodel);
    ~SpacecraftWrapper();
    void clbkSetClassCaps(FILEHANDLE cfg);
    void clbkPreStep(double SimT, double SimDT, double MJD);

    void AddMesh(rust::Str mesh_name) const;
    void AddMeshWithOffset(rust::Str mesh_name, const Vector3& ofs) const;
    void AddExhaust(rust::Str mesh_name) const;
    size_t AddExhaust(uintptr_t th, double lscale, double wscale) const;

    void SetPMI(const Vector3& pmi) const;
    void SetCameraOffset(const Vector3& co) const;
    void SetTouchdownPoints(const Vector3 &pt1, const Vector3 &pt2, const Vector3 &pt3) const;

    uintptr_t CreateThruster(const Vector3 &pos, const Vector3 &dir, double maxth0, uintptr_t ph, double isp) const;
    uintptr_t CreatePropellantResource(double mass) const;
    uintptr_t CreateThrusterGroup(rust::Slice<const uintptr_t> thrusters, THGROUP_TYPE thgroup_type) const;

    double GetPropellantMass(uintptr_t ph) const;
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
