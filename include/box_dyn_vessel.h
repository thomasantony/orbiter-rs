#pragma once
#include <cstdint>
#include <array>
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
