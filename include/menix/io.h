//? Data Input/Output abstraction

// Some architectures like x86 use ports to write/read data with their own
// instructions, while others like riscv64 write to memory mapped addresses.
// We need to be able to abstract this difference, therefore the
// [read,write][8,16,32,64] functions are declared as macros.
// The implementation for these functions should be defined in /arch/<arch>/io.c

#include <arch_bits.h>

#define read8(addr)			 read8(addr)
#define read16(addr)		 read16(addr)
#define read32(addr)		 read32(addr)
#define read64(addr)		 read64(addr)
#define write8(addr, value)	 write8(addr, value)
#define write16(addr, value) write16(addr, value)
#define write32(addr, value) write32(addr, value)
#define write64(addr, value) write64(addr, value)
