//? EDID decoding

#pragma once

#include <menix/common.h>

typedef struct ATTR(packed)
{
	uint8_t bytes[18];
} EdidDetailedTiming;

typedef struct ATTR(packed)
{
	// Header
	char			   header[8];
	uint16_t		   manufacturer_id;
	uint16_t		   product_id;
	uint32_t		   serial_no;
	uint8_t			   manufacture_week;
	uint8_t			   manufacture_year;
	uint8_t			   edid_version;
	uint8_t			   edid_revision;
	// Basic display parameters
	uint8_t			   video_input_params;
	uint8_t			   h_screen_size;
	uint8_t			   v_screen_size;
	uint8_t			   gamma;
	uint8_t			   features;
	// Chromaticity coordinates
	uint8_t			   red_green_least;
	uint8_t			   blue_white_least;
	uint8_t			   red_x_most;
	uint8_t			   red_y_most;
	uint8_t			   green_x_most;
	uint8_t			   green_y_most;
	uint8_t			   blue_x_most;
	uint8_t			   blue_y_most;
	uint8_t			   white_point_x;
	uint8_t			   white_point_y;
	// Timings
	bits			   timings:24;
	uint16_t		   standard_timings[8];
	// Timing descriptor
	EdidDetailedTiming timing_detail[4];
	// Extension flag and checksum
	uint8_t			   num_exts;
	uint8_t			   checksum;
} Edid;
