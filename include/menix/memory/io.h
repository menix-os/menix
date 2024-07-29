// MMIO read/write functions

#include <menix/common.h>

typedef volatile u8 mmio8_t;
typedef volatile u16 mmio16_t;
typedef volatile u32 mmio32_t;
#if CONFIG_bits >= 64
typedef volatile u64 mmio64_t;
#endif

// Macros for reading and writing data from and to memory mapped addresses.

#define read8(addr)			 (*((mmio8_t*)(addr)))
#define write8(addr, value)	 (*((mmio8_t*)(addr)) = (u8)(value))
#define read16(addr)		 (*((mmio16_t*)(addr)))
#define write16(addr, value) (*((mmio16_t*)(addr)) = (u16)(value))
#define read32(addr)		 (*((mmio32_t*)(addr)))
#define write32(addr, value) (*((mmio32_t*)(addr)) = (u32)(value))
#if CONFIG_bits >= 64
#define read64(addr)		 (*((mmio64_t*)(addr)))
#define write64(addr, value) (*((mmio64_t*)(addr)) = (u64)(value))
#endif
