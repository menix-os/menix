// RSDP/XSDT functions.

#include <menix/drv/acpi/acpi.h>
#include <menix/drv/acpi/types.h>
#include <menix/drv/pci/pci_acpi.h>
#include <menix/memory/alloc.h>
#include <menix/util/log.h>

#include <string.h>

static AcpiRsdt* rsdt;

// Performs a sanity check on a block of data.
static u8 acpi_checksum(void* ptr, usize size)
{
	// ACPI standard mandates that all data must add up to 0.
	u8 sum = 0;
	u8* cur = ptr;
	for (usize i = 0; i < size; i++)
		sum += cur[i];
	return sum;
}

void acpi_init(AcpiRsdp* rsdp)
{
	kassert(rsdp != NULL, "Failed to set RSDP: None given!");
	rsdt = ACPI_ADDR(rsdp->xsdt_address);

	acpi_log("Initialized ACPI (Rev. %u)\n", rsdp->revision);
}

void* acpi_find_table(const char* signature, usize index)
{
	usize num = 0;

	// Iterate over all tables.
	const usize num_entries = (rsdt->header.length - sizeof(AcpiDescHeader)) / 8;
	for (usize i = 0; i < num_entries; i++)
	{
		// Get the address to the next table.
		AcpiDescHeader* ptr = (void*)ACPI_ADDR(((u64*)rsdt->entries)[i]);
		// Check the signature.
		if (!acpi_checksum(ptr, ptr->length) && !memcmp(ptr->signature, signature, 4) && num++ == index)
			return (void*)ptr;
	}

	return NULL;
}
