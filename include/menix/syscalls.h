#ifndef MENIX_SYSCALLS_H
#define MENIX_SYSCALLS_H

#include <menix/archctl.h>
#include <menix/errno.h>
#include <menix/rights.h>
#include <menix/types.h>
#include <stddef.h>

// Panics and returns an error status to the parent process.
void menix_panic(menix_errno_t status);

void menix_log(const char* message, size_t length);

#endif
