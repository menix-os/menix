#ifndef _MENIX_IPC_H
#define _MENIX_IPC_H

#include <menix/object.h>
#include <menix/status.h>
#include <stddef.h>

enum {
    MENIX_LINK_OPTION_NONE = 0,
};

#ifndef __KERNEL__

// Creates a new link and returns two endpoint handles.
menix_status_t menix_link_create(uint32_t options, menix_obj_t endpoints[2]);

// Writes object handles and a message to a link endpoint.
menix_status_t menix_link_write(
    menix_obj_t endpoint,
    const menix_obj_t* handles,
    size_t num_handles,
    const void* data,
    size_t num_bytes
);

// Reads object handles and a message from a link endpoint.
menix_status_t menix_link_read(
    menix_obj_t endpoint,
    menix_obj_t* handle_buffer,
    size_t max_handles,
    size_t* actual_handles,
    void* data_buffer,
    size_t max_data,
    size_t* actual_data
);

#endif
#endif
