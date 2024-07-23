// x86 Serial interface

#include <menix/common.h>
#include <menix/serial.h>

#include <io.h>

#define COM1_BASE			 0x3F8	  // Serial port
#define DATA_REG			 0		  // Data Register
#define INT_ENABLE_REG		 1		  // Interrupt Enable Register
#define DIV_LSB				 0		  // Divisor Latch LSB
#define DIV_MSB				 1		  // Divisor Latch MSB
#define INT_ID_FIFO_CTRL_REG 2		  // Interrupt Identification and FIFO Control Register
#define LINE_CTRL_REG		 3		  // Line Control Register
#define MODEM_CTRL_REG		 4		  // Modem Control Register
#define LINE_STATUS_REG		 5		  // Line Status Register

#define TRANSMIT_FREE (arch_read8(COM1_BASE + LINE_STATUS_REG) & 0x20)

void serial_initialize()
{
	arch_write8(COM1_BASE + INT_ENABLE_REG, 0x00);			// Disable interrupts
	arch_write8(COM1_BASE + LINE_CTRL_REG, 0x80);			// Enable DLAB (set baud rate divisor)
	arch_write8(COM1_BASE + DIV_LSB, 0x03);					// Set divisor to 3 (lo byte) 38400 baud
	arch_write8(COM1_BASE + DIV_MSB, 0x00);					// Set divisor to 3 (hi byte)
	arch_write8(COM1_BASE + LINE_CTRL_REG, 0x03);			// 8 bits, no parity, one stop bit
	arch_write8(COM1_BASE + INT_ID_FIFO_CTRL_REG, 0xC7);	// Enable FIFO, clear them, with 14-byte threshold
	arch_write8(COM1_BASE + MODEM_CTRL_REG, 0x0B);			// IRQs enabled, RTS/DSR set
}

void serial_putchar(char c)
{
	// Wait for transmit to be empty.
	while (TRANSMIT_FREE == false)
		;
	switch (c)
	{
		case '\0': break;
		default: arch_write8(COM1_BASE + DATA_REG, c); break;
	}
}

void serial_write(const char* data, size_t size)
{
	for (size_t i = 0; i < size; i++)
		serial_putchar(data[i]);
}
