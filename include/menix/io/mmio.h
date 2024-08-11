// MMIO read/write functions

#include <menix/common.h>

typedef volatile u8 mmio8;
typedef volatile u16 mmio16;
typedef volatile u32 mmio32;
#if CONFIG_bits >= 64
typedef volatile u64 mmio64;
#endif

// Macros for reading and writing data from and to memory mapped addresses.

#define read8(addr)			 (*((mmio8*)(addr)))
#define write8(addr, value)	 (*((mmio8*)(addr)) = (u8)(value))
#define read16(addr)		 (*((mmio16*)(addr)))
#define write16(addr, value) (*((mmio16*)(addr)) = (u16)(value))
#define read32(addr)		 (*((mmio32*)(addr)))
#define write32(addr, value) (*((mmio32*)(addr)) = (u32)(value))
#if CONFIG_bits >= 64
#define read64(addr)		 (*((mmio64*)(addr)))
#define write64(addr, value) (*((mmio64*)(addr)) = (u64)(value))
#endif
