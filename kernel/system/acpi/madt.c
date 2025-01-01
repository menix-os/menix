// Multiple APIC Description Table

#include <menix/system/acpi/acpi.h>
#include <menix/system/acpi/madt.h>
#include <menix/util/log.h>

#include <uacpi/acpi.h>
#include <uacpi/tables.h>

struct acpi_madt* madt;
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

	uacpi_table madt_table;
	kassert(!uacpi_table_find_by_signature(ACPI_MADT_SIGNATURE, &madt_table),
			"ACPI tables don't contain a MADT! This is faulty behavior!");

	madt = madt_table.ptr;

	lapic_addr = madt->local_interrupt_controller_address;

	// Iterate over all APIC entries.
	for (u8* cur = (u8*)madt->entries; cur < (u8*)(madt) + madt->hdr.length; cur++)
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
	uacpi_table_unref(&madt_table);
}
