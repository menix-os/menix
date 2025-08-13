#ifndef _MENIX_IPC_H
#define _MENIX_IPC_H

#include <menix/object.h>
#include <menix/status.h>
#include <stddef.h>

#ifndef __KERNEL__

// Creates a new bi-directional link. It can be used to share messages with another process.
menix_status_t menix_link_create(menix_object_t* out_end0, menix_object_t* out_end1, size_t max_msg_size);

// Connects this client to the specified link. Returns two memory addresses for communicating with the other end.
// `send` is the outgoing buffer, while `recv` is the incoming buffer.
// Both buffers can only fit data with a length less or equal than the link's maximum data size.
menix_status_t menix_link_connect(menix_object_t link, void** out_send, const void** out_recv);

#endif
#endif
