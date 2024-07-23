// MMIO read/write functions

#include <menix/common.h>

typedef volatile uint8_t mmio8_t;
typedef volatile uint16_t mmio16_t;
typedef volatile uint32_t mmio32_t;
#ifdef CONFIG_64_bit
typedef volatile uint64_t mmio64_t;
#endif

// Macros for reading and writing data from and to memory mapped addresses.

#define read8(addr)			 (*((mmio8_t*)(addr)))
#define write8(addr, value)	 (*((mmio8_t*)(addr)) = (uint8_t)(value))
#define read16(addr)		 (*((mmio16_t*)(addr)))
#define write16(addr, value) (*((mmio16_t*)(addr)) = (uint16_t)(value))
#define read32(addr)		 (*((mmio32_t*)(addr)))
#define write32(addr, value) (*((mmio32_t*)(addr)) = (uint32_t)(value))
#ifdef CONFIG_64_bit
#define read64(addr)		 (*((mmio64_t*)(addr)))
#define write64(addr, value) (*((mmio64_t*)(addr)) = (uint64_t)(value))
#endif
