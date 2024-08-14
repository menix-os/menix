// ACPI Structures according to v6.4

#pragma once
#include <menix/common.h>

#define ACPI_DESCRIPTION_HEADER \
	struct \
	{ \
		char signature[4]; \
		u32 length; \
		u8 revision; \
		u8 checksum; \
		char oemid[6]; \
		char oem_table_id[8]; \
		u32 oem_revision; \
		u32 creator_id; \
		u32 creator_revision; \
	}

// Root System Description Pointer
typedef struct
{
	char signature[8];
	u8 checksum;
	char oemid[6];
	u8 revision;
	u32 rsdt_address;
	u32 length;
	u64 xsdt_address;
	u8 ext_checksum;
	u8 reserved[3];
} ATTR(packed) AcpiRsdp;

// Extended System Description Table
typedef struct
{
	ACPI_DESCRIPTION_HEADER;
	u64* entry;
} ATTR(packed) AcpiXsdt;
