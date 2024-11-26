// High precision event timer table.

#pragma once
#include <menix/common.h>
#include <menix/system/acpi/types.h>

typedef struct
{
	u8 address_space_id;
	u8 register_bit_width;
	u8 register_bit_offset;
	u8 reserved;
	u64 address;
} ATTR(packed) AcpiAddr;

typedef struct
{
	AcpiDescHeader header;
	u8 hardware_rev_id;
	Bits num_comparators:5;
	Bits counter_size:1;
	Bits reserved:1;
	Bits legacy_replacement:1;
	u16 pci_vendor;
	AcpiAddr address;
	u8 hpet_number;
	u16 minimum_tick;
	u8 page_protection;
} ATTR(packed) AcpiHpet;

// HPET registers
typedef struct
{
	u64 capabilities;
	u64 _pad0;
	u64 configuration;
	u64 _pad1;
	u64 interrupt_status;
	u64 _pad2[0x19];
	u64 main_counter;
	u64 _pad3;
} ATTR(packed) HpetRegisters;

typedef struct
{
	volatile HpetRegisters* regs;

} Hpet;

void hpet_init();
