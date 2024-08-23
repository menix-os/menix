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

// Root System Description Pointer
typedef struct
{
	char signature[8];
	u8 checksum;
	char oemid[6];
	u8 revision;
	u32 rsdt_address;
	u32 length;
#if CONFIG_bits >= 64
	u64 xsdt_address;
	u8 ext_checksum;
	u8 reserved[3];
#endif
} ATTR(packed) AcpiRsdp;

// Root System Description Table
typedef struct
{
	AcpiDescHeader header;
	PhysAddr entries[];
} ATTR(packed) AcpiRsdt;

// Multiple APIC Table
typedef struct
{
	AcpiDescHeader header;
	u32 lapic_addr;
	u32 flags;

} ATTR(packed) AcpiMadt;

typedef struct
{
	u64 base;
	u16 segment_group;
	u8 bus_start;
	u8 bus_end;
	char reserved[4];
} ATTR(packed) AcpiMcfgEntry;

// Enhanced Configuration Mechanism
typedef struct
{
	AcpiDescHeader header;
	char reserved[8];
	AcpiMcfgEntry entries[];
} ATTR(packed) AcpiMcfg;

// Boot Graphics Record Table
typedef struct
{
	AcpiDescHeader header;
	u16 version_id;
	u8 status;
	u8 image_type;
	PhysAddr image_addr;
	u32 image_xoff;
	u32 image_yoff;
} ATTR(packed) AcpiBgrt;
