// High precision event timer table.

#pragma once
#include <menix/common.h>

// HPET registers
typedef struct [[gnu::packed]]
{
	u64 capabilities;
	u64 _pad0;
	u64 configuration;
	u64 _pad1;
	u64 interrupt_status;
	u64 _pad2[0x19];
	u64 main_counter;
	u64 _pad3;
} HpetRegisters;

void hpet_init();
