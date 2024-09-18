// System firmware and platform initialization

#pragma once

#include <menix/system/boot.h>

void fw_init(BootInfo* info);

BootInfo* fw_get_boot_info();
