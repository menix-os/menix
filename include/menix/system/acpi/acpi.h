// ACPI functions

#pragma once
#include <menix/common.h>

// Initializes the ACPI subsystem with a pointer to the RSDP.
void acpi_init(PhysAddr rsdp);
