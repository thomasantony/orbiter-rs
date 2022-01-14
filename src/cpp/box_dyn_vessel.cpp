#include "box_dyn_vessel.h"
#include "orbiter-rs/src/ffi.rs.h"

BoxDynVessel::BoxDynVessel() noexcept : repr({0, 0}) {}

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

BoxDynVessel &BoxDynVessel::operator=(BoxDynVessel && other)
{
    repr = other.repr;
    other.repr = {0, 0};
    return *this;
}
