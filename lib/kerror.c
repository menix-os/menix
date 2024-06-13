/*-----------------
Kernel error output
-----------------*/

#include <menix/error.h>
#include <menix/stdio.h>

void kerror(const char* str)
{
	// If we have a message, print it.
	// Otherwise, we don't know.
	printf("[ERR]\t%s\n", str ? str : "Unknown error!");
}
