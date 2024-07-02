//? Limine bootloader entry point.

#include <menix/common.h>
#include <menix/boot.h>
#include "limine.h"

ATTR(used) ATTR(section(".requests")) static volatile LIMINE_BASE_REVISION(2);

ATTR(used) ATTR(section(".requests")) static volatile struct limine_framebuffer_request framebuffer_request = {
	.id = LIMINE_FRAMEBUFFER_REQUEST,
	.revision = 0
};

ATTR(used) ATTR(section(".requests_start_marker")) static volatile LIMINE_REQUESTS_START_MARKER;
ATTR(used) ATTR(section(".requests_end_marker")) static volatile LIMINE_REQUESTS_END_MARKER;

void kernel_boot()
{
	kernel_main();
}
