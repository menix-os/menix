#ifndef _MENIX_SYSTEM_H
#define _MENIX_SYSTEM_H

#include <menix/status.h>
#include <stddef.h>

// Stops execution of this program.
[[noreturn]]
void menix_panic(menix_status_t error);

// Writes a buffer to the output stream.
void menix_log(const char* message, size_t length);

#endif
