#pragma once
// DO NOT IMPORT lib.rs.h here
#include "rust/cxx.h"
#include "orbitersdk.h"
#include "box_wrapper.h"

void debugLog(rust::Str);

struct Vector3;

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
    void SetPMI(const Vector3& pmi) const;
    // double CalcEmptyMass();
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
