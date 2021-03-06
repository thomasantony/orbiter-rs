#pragma once
// DO NOT IMPORT lib.rs.h here
#include "rust/cxx.h"
#include "orbitersdk.h"
#include "box_dyn_vessel.h"

void ODebug(rust::String);

using Vector3 = VECTOR3;
class VesselContext;
using VesselInitFn = rust::Fn<BoxDynVessel(VesselContext &)>;

using c_void = void;

// Wrapper for oapiCreateVessel
OBJHANDLE oapi_create_vessel(rust::String name, rust::String classname, const VESSELSTATUS &status);
// VESSEL *vessel_ovcInit(OBJHANDLE hvessel, int flightmodel, BoxDynVessel box_vessel);
VESSEL *vessel_ovcInit(OBJHANDLE hvessel, int flightmodel, VesselInitFn fn);
void vessel_ovcExit(VESSEL *vessel);


// ==============================================================
// Spacecraft class interface
// ==============================================================
class VesselContext : public VESSEL4
{
public:
    // VesselContext(OBJHANDLE hVessel, int flightmodel, BoxDynVessel& box_vessel);
    VesselContext(OBJHANDLE hVessel, int flightmodel, VesselInitFn fn);
    ~VesselContext();
    void clbkSetClassCaps(FILEHANDLE cfg);
    void clbkPreStep(double SimT, double SimDT, double MJD);
    void clbkPostStep(double SimT, double SimDT, double MJD);
    void clbkLoadStateEx(FILEHANDLE scn, void* status);
    void clbkSaveState(FILEHANDLE scn);
    int clbkConsumeBufferedKey(DWORD key, bool down, char *kstate);

    void AddMesh(rust::String mesh_name) const;
    void AddMeshWithOffset(rust::String mesh_name, const Vector3& ofs) const;
    size_t AddExhaust(THRUSTER_HANDLE th, double lscale, double wscale) const;

    void ParseScenarioLineEx(char* line, void* status) const;
    THRUSTER_HANDLE CreateThruster(const Vector3 &pos, const Vector3 &dir, double maxth0, PROPELLANT_HANDLE ph, double isp) const;
    PROPELLANT_HANDLE CreatePropellantResource(double mass) const;
    THGROUP_HANDLE CreateThrusterGroup(rust::Slice<const THRUSTER_HANDLE> thrusters, THGROUP_TYPE thgroup_type) const;

    rust::Str GetName() const;
    OBJHANDLE GetSurfaceRef() const;
private:
    BoxDynVessel rust_spacecraft_;
    VesselInitFn rust_init_fn_;
};
BoxDynVessel vessel_init(VesselContext& vessel);
