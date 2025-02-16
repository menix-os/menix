// x86 Serial interface

#include <menix/common.h>
#include <menix/system/logger.h>
#include <menix/util/log.h>

#include <io.h>
#include <serial.h>

#define COM1_BASE			 0x3F8	  // Serial port
#define DATA_REG			 0		  // Data Register
#define INT_ENABLE_REG		 1		  // Interrupt Enable Register
#define DIV_LSB				 0		  // Divisor Latch LSB
#define DIV_MSB				 1		  // Divisor Latch MSB
#define INT_ID_FIFO_CTRL_REG 2		  // Interrupt Identification and FIFO Control Register
#define LINE_CTRL_REG		 3		  // Line Control Register
#define MODEM_CTRL_REG		 4		  // Modem Control Register
#define LINE_STATUS_REG		 5		  // Line Status Register

#define TRANSMIT_FREE (asm_read8(COM1_BASE + LINE_STATUS_REG) & 0x20)

// If the COM port works or not.
static bool can_use_serial = false;

static void serial_putchar(char c)
{
	if (!can_use_serial)
		return;

	// Wait until we can send things.
	while (TRANSMIT_FREE == false)
		;

	switch (c)
	{
		case '\0': break;	 // Don't transmit null terminators.
		default: asm_write8(COM1_BASE + DATA_REG, c); break;
	}
}

static isize serial_write(const void* data, usize size)
{
	for (usize i = 0; i < size; i++)
		serial_putchar(((char*)data)[i]);

	return size;
}

void serial_init()
{
	asm_write8(COM1_BASE + INT_ENABLE_REG, 0x00);		   // Disable interrupts
	asm_write8(COM1_BASE + LINE_CTRL_REG, 0x80);		   // Enable DLAB (set baud rate divisor)
	asm_write8(COM1_BASE + DIV_LSB, 0x03);				   // Set divisor to 3 (lo byte) 38400 baud
	asm_write8(COM1_BASE + DIV_MSB, 0x00);				   // Set divisor to 3 (hi byte)
	asm_write8(COM1_BASE + LINE_CTRL_REG, 0x03);		   // 8 bits, no parity, one stop bit
	asm_write8(COM1_BASE + INT_ID_FIFO_CTRL_REG, 0xC7);	   // Enable FIFO, clear them, with 14-byte threshold
	asm_write8(COM1_BASE + MODEM_CTRL_REG, 0x0B);		   // IRQs enabled, RTS/DSR set

	asm_write8(COM1_BASE + MODEM_CTRL_REG, 0x1E);	 // Set to loopback mode for testing.
	asm_write8(COM1_BASE + DATA_REG, 0xAE);			 // Send a test byte.
	if (asm_read8(COM1_BASE + DATA_REG) == 0xAE)	 // If we get the same back, we're ready.
	{
		can_use_serial = true;
		asm_write8(COM1_BASE + MODEM_CTRL_REG, 0x0F);	 // Set back to normal operation mode.
		logger_register("com1", serial_write);
	}
}
