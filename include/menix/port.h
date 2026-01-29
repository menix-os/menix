#ifndef MENIX_PORT_H
#define MENIX_PORT_H

#include <menix/errno.h>
#include <menix/handle.h>

enum menix_port_flags {
    MENIX_PORT_FLAG_NONE = 0,
    // Allow sending messages even if one endpoint is not connected.
    MENIX_PORT_FLAG_ALLOW_UNCONNECTED = 1 << 0,
};

typedef menix_errno_t (*menix_port_action_t)(menix_handle_t);

#ifndef __KERNEL__

// Creates a new port.
menix_errno_t menix_port_create(enum menix_port_flags flags, menix_handle_t* endpoint0, menix_handle_t* endpoint1);

// Maps the message buffer in the address space and returns its base address.
// There may only be one message buffer per thread.
menix_errno_t menix_port_connect(
    menix_handle_t port,
    size_t num_handles,
    size_t num_bytes,
    menix_handle_t** out_handle_buf,
    void** out_data_buf
);

menix_errno_t menix_port_action(menix_handle_t port, menix_port_action_t action);

menix_errno_t menix_port_write(menix_handle_t port);

#endif

#endif
