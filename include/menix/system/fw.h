// System firmware and platform initialization

#pragma once

#include <menix/system/boot.h>

// Initializes firmware. Must be called after `arch_early_init` and before `arch_init`.
void fw_init(BootInfo* info);

BootInfo* fw_get_boot_info();
