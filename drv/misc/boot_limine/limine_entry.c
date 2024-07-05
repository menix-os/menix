//? Limine bootloader entry point.

#include <menix/arch.h>
#include <menix/boot.h>
#include <menix/common.h>

#include "limine.h"

#define LIMINE_REQUEST ATTR(section(".requests")) ATTR(used) static volatile

// Start requests
ATTR(used) ATTR(section(".requests_start_marker")) static volatile LIMINE_REQUESTS_START_MARKER;

LIMINE_REQUEST LIMINE_BASE_REVISION(2);

LIMINE_REQUEST struct limine_framebuffer_request framebuffer_request = {
	.id = LIMINE_FRAMEBUFFER_REQUEST,
	.revision = 0,
};
LIMINE_REQUEST struct limine_boot_time_request time_request = {
	.id = LIMINE_BOOT_TIME_REQUEST,
	.revision = 0,
};

// End requests
ATTR(used) ATTR(section(".requests_end_marker")) static volatile LIMINE_REQUESTS_END_MARKER;

void kernel_boot()
{
	kernel_main();
}
