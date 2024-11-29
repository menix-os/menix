// RSDP/XSDT functions.

#include <menix/memory/alloc.h>
#include <menix/system/acpi/acpi.h>
#include <menix/system/acpi/madt.h>
#include <menix/system/acpi/mcfg.h>
#include <menix/system/acpi/types.h>
#include <menix/util/log.h>

#include <string.h>

#ifdef CONFIG_arch_x86_64
#include <hpet.h>
#endif

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

	// Check if we have ACPI 2.0+ available. If yes, use the XSDT instead of the RSDT.
	if (rsdp->revision >= 2)
		rsdt = ACPI_ADDR(rsdp->xsdt_address);
	else
		rsdt = ACPI_ADDR(rsdp->rsdt_address);

	print_log("acpi: ACPI Revision %hhu\n", rsdp->revision);

	// Initialize architecture dependent tables.
#ifdef CONFIG_arch_x86_64
	hpet_init();
#endif

	// Initialize independent tables.
	madt_init();

#ifdef CONFIG_pci
	mcfg_init();
#endif

	print_log("acpi: Initialized ACPI\n");
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
