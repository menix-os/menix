// x86 Serial interface

#include <menix/common.h>
#include <menix/io/terminal.h>

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

#define TRANSMIT_FREE (arch_x86_read8(COM1_BASE + LINE_STATUS_REG) & 0x20)

// If the COM port works or not.
static bool can_use_serial = false;

static Handle serial_driver;

void serial_init()
{
	arch_x86_write8(COM1_BASE + INT_ENABLE_REG, 0x00);			// Disable interrupts
	arch_x86_write8(COM1_BASE + LINE_CTRL_REG, 0x80);			// Enable DLAB (set baud rate divisor)
	arch_x86_write8(COM1_BASE + DIV_LSB, 0x03);					// Set divisor to 3 (lo byte) 38400 baud
	arch_x86_write8(COM1_BASE + DIV_MSB, 0x00);					// Set divisor to 3 (hi byte)
	arch_x86_write8(COM1_BASE + LINE_CTRL_REG, 0x03);			// 8 bits, no parity, one stop bit
	arch_x86_write8(COM1_BASE + INT_ID_FIFO_CTRL_REG, 0xC7);	// Enable FIFO, clear them, with 14-byte threshold
	arch_x86_write8(COM1_BASE + MODEM_CTRL_REG, 0x0B);			// IRQs enabled, RTS/DSR set

	arch_x86_write8(COM1_BASE + MODEM_CTRL_REG, 0x1E);	  // Set to loopback mode for testing.
	arch_x86_write8(COM1_BASE + DATA_REG, 0xAE);		  // Send a test byte.
	if (arch_x86_read8(COM1_BASE + DATA_REG) == 0xAE)	  // If we get the same back, we're ready.
	{
		can_use_serial = true;
		arch_x86_write8(COM1_BASE + MODEM_CTRL_REG, 0x0F);	  // Set back to normal operation mode.
	}

	if (can_use_serial)
		terminal_set_driver(0, &serial_driver);
}

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
		default: arch_x86_write8(COM1_BASE + DATA_REG, c); break;
	}
}

static isize serial_write(Handle* handle, FileDescriptor* fd, const void* data, usize size, off_t off)
{
	for (usize i = 0; i < size; i++)
		serial_putchar(((char*)data)[i]);

	return size;
}

static char serial_getchar()
{
	if (!can_use_serial)
		return '\0';

	// Wait until we can send things.
	while (TRANSMIT_FREE == false)
		;

	return arch_x86_read8(COM1_BASE + DATA_REG);
}

static isize serial_read(Handle* handle, FileDescriptor* fd, void* data, usize size, off_t off)
{
	((char*)data)[0] = serial_getchar();

	return 1;
}

static Handle serial_driver = {
	.write = serial_write,
	.read = serial_read,
};
