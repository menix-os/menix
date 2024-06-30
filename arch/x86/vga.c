//? x86 VGA mode serial interface

#include <menix/serial.h>
#include <stdint.h>
#include <string.h>

#define VGA_WIDTH  80
#define VGA_HEIGHT 25

static uint16_t* volatile const VGA_MEMORY = (uint16_t*)0xB8000;

static size_t  vga_row;
static size_t  vga_column;
static uint8_t vga_current_col;

enum vga_color
{
	VGA_COLOR_BLACK = 0,
	VGA_COLOR_BLUE = 1,
	VGA_COLOR_GREEN = 2,
	VGA_COLOR_CYAN = 3,
	VGA_COLOR_RED = 4,
	VGA_COLOR_MAGENTA = 5,
	VGA_COLOR_BROWN = 6,
	VGA_COLOR_LIGHT_GREY = 7,
	VGA_COLOR_DARK_GREY = 8,
	VGA_COLOR_LIGHT_BLUE = 9,
	VGA_COLOR_LIGHT_GREEN = 10,
	VGA_COLOR_LIGHT_CYAN = 11,
	VGA_COLOR_LIGHT_RED = 12,
	VGA_COLOR_LIGHT_MAGENTA = 13,
	VGA_COLOR_LIGHT_BROWN = 14,
	VGA_COLOR_WHITE = 15,
};

static inline uint8_t vga_entry_color(enum vga_color fg, enum vga_color bg)
{
	return fg | bg << 4;
}

static inline uint16_t vga_entry(uint8_t uc, uint8_t color)
{
	return (uint16_t)uc | (uint16_t)color << 8;
}

void serial_initialize()
{
	vga_row = 0;
	vga_column = 0;
	vga_current_col = vga_entry_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
	for (size_t y = 0; y < VGA_HEIGHT; y++)
	{
		for (size_t x = 0; x < VGA_WIDTH; x++)
		{
			const size_t index = y * VGA_WIDTH + x;
			VGA_MEMORY[index] = vga_entry(' ', vga_current_col);
		}
	}
}

void vga_putentryat(uint8_t c, uint8_t color, size_t x, size_t y)
{
	const size_t index = y * VGA_WIDTH + x;
	VGA_MEMORY[index] = vga_entry(c, color);
}

void serial_putchar(char c)
{
	switch (c)
	{
		case '\0':
			break;
		case '\n':
			vga_column = 0;
			vga_row++;
			break;
		case '\t':
			vga_column += 4 - vga_column % 4;
			break;
		case '\r':
			vga_column = 0;
			break;
		case '\b':
			if (vga_column > 0)
				vga_column -= 1;
			vga_putentryat(' ', vga_current_col, vga_column, vga_row);
			break;
		default:
			vga_putentryat((uint8_t)c, vga_current_col, vga_column, vga_row);
			vga_column++;
			break;
	}

	// If at the end of the line, wrap to the next.
	if (vga_column + 1 >= VGA_WIDTH)
	{
		vga_column = 0;
		vga_row++;
	}

	// If at the bottom of the VGA buffer, scroll all lines.
	if (vga_row >= VGA_HEIGHT)
	{
		// Move each line up, starting with the second line.
		memmove(VGA_MEMORY, VGA_MEMORY + (VGA_WIDTH * 2), (VGA_WIDTH * 2) * (VGA_HEIGHT - 1));

		// Clear the last line.
		vga_row -= 2;	 // 1: OOB line. 2: Moved line.
		for (uint8_t i = 0; i < VGA_WIDTH; i++)
			vga_putentryat(' ', vga_current_col, i, VGA_HEIGHT - 1);
	}
}

void serial_write(const char* data, size_t size)
{
	for (size_t i = 0; i < size; i++)
		serial_putchar(data[i]);
}

void serial_writestring(const char* data)
{
	serial_write(data, strlen(data));
}
