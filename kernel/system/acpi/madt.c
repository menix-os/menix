// Multiple APIC Description Table

#include <menix/system/acpi/acpi.h>
#include <menix/system/acpi/madt.h>
#include <menix/util/log.h>

AcpiMadt* acpi_madt;
PhysAddr lapic_addr;

MadtLApicList madt_lapic_list;
MadtIoApicList madt_ioapic_list;
MadtIsoList madt_iso_list;
MadtNmiList madt_nmi_list;

void madt_init()
{
	list_new(madt_lapic_list, 0);
	list_new(madt_ioapic_list, 0);
	list_new(madt_iso_list, 0);
	list_new(madt_nmi_list, 0);

	acpi_madt = acpi_find_table("APIC", 0);
	kassert(acpi_madt != NULL, "ACPI tables don't contain a MADT! This is faulty behavior!");

	lapic_addr = acpi_madt->lapic_addr;

	// Iterate over all APIC entries.
	for (u8* cur = acpi_madt->entries; cur < (u8*)(acpi_madt + acpi_madt->header.length); cur++)
	{
		// First field is the type.
		switch (*cur)
		{
			// Local APIC
			case 0: list_push(&madt_lapic_list, (void*)cur); break;
			// IOAPIC
			case 1: list_push(&madt_ioapic_list, (void*)cur); break;
			// Interrupt source override
			case 2: list_push(&madt_iso_list, (void*)cur); break;
			// Non maskable interrupt source
			case 4: list_push(&madt_nmi_list, (void*)cur); break;
			// 64-bit address override
			case 5: lapic_addr = ((MadtLApicAddr*)cur)->lapic_addr; break;
		}
	}
}
