//? Data Input/Output abstraction

//? Some architectures like x86 use ports to write/read data with their own
//? instructions, while others like riscv64 write to special mapped addresses.
//? We need to be able to abstract this difference, therefore the
//? [read,write][8,16,32,64] functions are declared as aliases to macros.

#define read8	arch_read8
#define read16	arch_read16
#define read32	arch_read32
#define read64	arch_read64
#define write8	arch_write8
#define write16 arch_write16
#define write32 arch_write32
#define write64 arch_write64
