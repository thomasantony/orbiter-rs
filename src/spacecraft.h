#pragma once
// DO NOT IMPORT lib.rs.h here
#include "rust/cxx.h"
#include "orbitersdk.h"
#include "box_wrapper.h"

void debugLog(rust::Str);

// struct RustSpacecraft;
// struct DynVessel;
// struct BoxDynVessel;

class BoxDynVessel
{
public:
    BoxDynVessel(BoxDynVessel &&) noexcept;
    ~BoxDynVessel() noexcept;
    using IsRelocatable = std::true_type;

    void set_class_caps() const;
    void pre_step(double SimT, double SimDT, double MJD);

private:
    std::array<std::uintptr_t, 2> repr;
};
using PtrBoxDynVessel = BoxDynVessel *;
// ==============================================================
// Spacecraft class interface
// ==============================================================
class RustySpace : public VESSEL3
{
public:
    RustySpace(OBJHANDLE hVessel, int flightmodel);
    ~RustySpace();
    void clbkSetClassCaps(FILEHANDLE cfg);
    void clbkPreStep(double SimT, double SimDT, double MJD);

    // double CalcEmptyMass();
    // int clbkConsumeBufferedKey(DWORD key, bool down, char *kstate);

    // void SetupMeshes();
    // void AddLanderMesh();
    // void AddRetroMesh();
    // void AddAMRMesh();

    // void SpawnObject(char *classname, char *ext, VECTOR3 ofs);
    // void Jettison();

private:
    // std::unique_ptr<DynVessel, BoxDeleter<DynVessel>> rust_spacecraft_; // Reference to rust struct instance
    BoxDynVessel rust_spacecraft_;
    // THRUSTER_HANDLE th_vernier[3], th_retro, th_rcs[6], th_group[2];
    // PROPELLANT_HANDLE ph_vernier, ph_rcs, ph_retro;
};
