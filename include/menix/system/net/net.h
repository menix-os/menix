// Network packet sending/receiving

#pragma once
#include <menix/common.h>

// Describes a Layer 2 ethernet frame.
typedef struct
{
	u8 dst[6];		 // MAC destination
	u8 src[6];		 // MAC source
	u16 type;		 // Ethertype
	u8 payload[];	 // 42-1500 bytes, followed by a 32-bit CRC.
} ATTR(packed) EthernetFrame;
