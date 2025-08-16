#include <menix/ipc.h>
#include <menix/object.h>
#include <menix/status.h>
#include <stddef.h>
#include <stdint.h>

menix_status_t menix_link_create(uint32_t options, menix_obj_t endpoints[2]) {
    // TODO
    return MENIX_OK;
}

menix_status_t menix_link_write(
    menix_obj_t endpoint,
    const menix_obj_t* handles,
    size_t num_handles,
    const void* data,
    size_t num_bytes
);

menix_status_t menix_link_read(
    menix_obj_t endpoint,
    menix_obj_t* handle_buffer,
    size_t max_handles,
    size_t* actual_handles,
    void* data_buffer,
    size_t max_data,
    size_t* actual_data
);
