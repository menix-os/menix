// ACPI functions

#pragma once
#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/system/acpi/types.h>

// Converts an ACPI physical address to a virtual one.
#define ACPI_ADDR(addr) ((PhysAddr)(addr) + pm_get_phys_base())

// Initializes the ACPI subsystem with a pointer to the RSDP.
void acpi_init(PhysAddr rsdp);

// Finds a table using its signature.
// `signature`: A 4-character string with the table's signature.
// `index`: The nth instance of a table with the given signature.
void* acpi_find_table(const char* signature, usize index);
