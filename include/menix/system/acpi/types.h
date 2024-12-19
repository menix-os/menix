// ACPI structs and enums according to v6.5

#pragma once
#include <menix/common.h>

typedef struct
{
	char signature[4];
	u32 length;
	u8 revision;
	u8 checksum;
	char oemid[6];
	char oem_table_id[8];
	u32 oem_revision;
	u32 creator_id;
	u32 creator_revision;
} AcpiDescHeader;

// Multiple APIC Table
typedef struct
{
	AcpiDescHeader header;
	u32 lapic_addr;
	u32 flags;
	u8 entries[];
} ATTR(packed) AcpiMadt;
