#ifndef _MENIX_IPC_H
#define _MENIX_IPC_H

#include <menix/handle.h>
#include <stddef.h>

#define MENIX_PORT_DEFAULT 0

enum menix_port_flags {
    MENIX_PORT_FLAG_NONE = 0,
    // Allow sending messages even if one endpoint is not connected.
    MENIX_PORT_FLAG_ALLOW_UNCONNECTED = 1 << 0,
};

#endif
