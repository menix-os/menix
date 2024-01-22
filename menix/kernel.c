#include <menix/tty.h>

void kernel_main(void)
{
	// Init terminal.
	terminal_initialize();

	terminal_writestring("Hello world!\n");
	terminal_setcolor(13);
	terminal_writestring("This works\tagain!\n");
}