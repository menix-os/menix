// Network packet sending/receiving

#pragma once
#include <menix/common.h>

typedef struct
{
	u8 octets[6];
} MacAddress;

// Describes a Layer 2 ethernet frame.
typedef struct
{
	MacAddress dst;	   // MAC destination
	MacAddress src;	   // MAC source
	u16 type;		   // Ethertype
	u8 payload[];	   // 42-1500 bytes, followed by a 32-bit CRC.
} ATTR(packed) EthernetFrame;
