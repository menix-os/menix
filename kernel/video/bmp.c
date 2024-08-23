// BMP file format utilities.

#include <menix/video/bmp.h>

void bmp_unpack24_to_32(u8* rgba, const BmpHeader* bmp)
{
	if (!rgba || !bmp)
		return;

	const u32 width = bmp->dib.width;								  // Width in pixels
	const u32 height = bmp->dib.height;								  // Height in pixels
	const u32 cpp = (bmp->dib.bpp / 8);								  // Bytes per pixel
	const u32 byte_width = width * cpp;								  // Length of one line in bytes.
	const u32 pitch = byte_width + (4 - (byte_width % 4) % 4);		  // Pitch per one line (BMPs are 4 byte padded).
	u8* rgb = ((void*)bmp + bmp->offset) + (pitch * (height - 1));	  // Go to the start of the last line and write up.
	for (usize y = 0; y < height; y++)
	{
		for (usize x = 0; x < byte_width; x += cpp)
		{
			rgba[0] = rgb[x];		 // Blue
			rgba[1] = rgb[x + 1];	 // Green
			rgba[2] = rgb[x + 2];	 // Red
			rgba[3] = 0xFF;			 // Alpha
			rgba += 4;
		}
		rgb -= pitch;
	}
}
