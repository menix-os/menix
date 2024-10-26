// Network driver abstractions

#pragma once
#include <menix/common.h>
#include <menix/system/net/net.h>

typedef struct
{
	// Sends a buffer to `destination`. Returns `true` on success.
	bool (*send_packet)(MacAddress destination, Buffer input);
	// Handles incoming packets. Returns `true` on success and stores the data in `output` if nonzero.
	bool (*receive_packet)(MacAddress source, Buffer* output);
} NetworkDriver;
