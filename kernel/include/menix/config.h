/*------------------
Configuration system
------------------*/

#pragma once

#include <generated/config.h>
#include <generated/exports.h>

// Check if an option has been set.
#define CFG_ENABLED(option) defined(CFG_##option)
