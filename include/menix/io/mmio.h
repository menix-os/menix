// MMIO read/write functions

#include <menix/common.h>

typedef volatile u8 mmio8;
typedef volatile u16 mmio16;
typedef volatile u32 mmio32;
#if MENIX_BITS >= 64
typedef volatile u64 mmio64;
#endif
typedef volatile usize mmiosize;

// Macros for reading and writing data from and to memory mapped addresses.

#define mmio_read8(addr)		  (*((mmio8*)(addr)))
#define mmio_write8(addr, value)  (*((mmio8*)(addr)) = (u8)(value))
#define mmio_read16(addr)		  (*((mmio16*)(addr)))
#define mmio_write16(addr, value) (*((mmio16*)(addr)) = (u16)(value))
#define mmio_read32(addr)		  (*((mmio32*)(addr)))
#define mmio_write32(addr, value) (*((mmio32*)(addr)) = (u32)(value))
#if MENIX_BITS >= 64
#define mmio_read64(addr)		  (*((mmio64*)(addr)))
#define mmio_write64(addr, value) (*((mmio64*)(addr)) = (u64)(value))
#endif

#define mmio_readsize(addr)			(*((mmiosize*)(addr)))
#define mmio_writesize(addr, value) (*((mmiosize*)(addr)) = (usize)(value))
