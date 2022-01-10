
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

RustySpace::RustySpace(OBJHANDLE hVessel, int flightmodel)
    : VESSEL3(hVessel, flightmodel),
      // rust_spacecraft_(box_to_uptr(create_rust_spacecraft()))
      rust_spacecraft_(std::move(create_rust_spacecraft()))
{
}

RustySpace::~RustySpace()
{
}

void RustySpace::clbkSetClassCaps(FILEHANDLE cfg)
{
    // physical vessel parameters
    SetSize(1.0);
    SetPMI(_V(0.50, 0.50, 0.50));
    AddMesh("ShuttlePB");
    rust_spacecraft_.set_class_caps();
}

// Pre-step logic for differential thrust
void RustySpace::clbkPreStep(double SimT, double SimDT, double MJD)
{
    rust_spacecraft_.pre_step(SimT, SimDT, MJD);
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

void BoxDynVessel::set_class_caps() const
{
    dyn_vessel_set_class_caps(*this);
}

void BoxDynVessel::pre_step(double SimT, double SimDT, double MJD)
{
    dyn_vessel_pre_step(*this, SimT, SimDT, MJD);
}


// ==============================================================
// API callback interface
// ==============================================================

// --------------------------------------------------------------
// Vessel initialisation
// --------------------------------------------------------------
DLLCLBK VESSEL *ovcInit(OBJHANDLE hvessel, int flightmodel)
{
    return new RustySpace(hvessel, flightmodel);
}

// --------------------------------------------------------------
// Vessel cleanup
// --------------------------------------------------------------
DLLCLBK void ovcExit(VESSEL *vessel)
{
    if (vessel)
        delete (RustySpace *)vessel;
}
