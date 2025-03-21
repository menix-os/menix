#pragma once

#include <menix/common.h>
#include <menix/util/list.h>

// MADT entry types
typedef struct [[gnu::packed]]
{
	u8 type;
	u8 length;
	u8 acpi_id;
	u8 lapic_id;
	u32 flags;
} MadtLApic;

typedef struct [[gnu::packed]]
{
	u8 type;
	u8 length;
	u8 ioapic_id;
	u8 reserved;
	u32 ioapic_addr;
	u32 gsi_base;
} MadtIoApic;

typedef struct [[gnu::packed]]
{
	u8 type;
	u8 length;
	u8 bus_source;
	u8 irq_source;
	u32 gsi;
	u16 flags;
} MadtIso;

typedef struct [[gnu::packed]]
{
	u8 type;
	u8 length;
	u8 acpi_id;
	u16 flags;
	u8 lint;
} MadtNmi;

typedef struct [[gnu::packed]]
{
	u8 type;
	u8 length;
	u16 reserved;
	u64 lapic_addr;
} MadtLApicAddr;

typedef List(MadtLApic*) MadtLApicList;
typedef List(MadtIoApic*) MadtIoApicList;
typedef List(MadtIso*) MadtIsoList;
typedef List(MadtNmi*) MadtNmiList;

extern MadtLApicList madt_lapic_list;
extern MadtIoApicList madt_ioapic_list;
extern MadtIsoList madt_iso_list;
extern MadtNmiList madt_nmi_list;

void madt_init();
