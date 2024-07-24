// EDID decoding

#pragma once

#include <menix/common.h>

typedef struct ATTR(packed)
{
	u8 bytes[18];
} EdidDetailedTiming;

typedef struct ATTR(packed)
{
	// Header
	char header[8];
	u16 manufacturer_id;
	u16 product_id;
	u32 serial_no;
	u8 manufacture_week;
	u8 manufacture_year;
	u8 edid_version;
	u8 edid_revision;
	// Basic display parameters
	u8 video_input_params;
	u8 h_screen_size;
	u8 v_screen_size;
	u8 gamma;
	u8 features;
	// Chromaticity coordinates
	u8 red_green_least;
	u8 blue_white_least;
	u8 red_x_most;
	u8 red_y_most;
	u8 green_x_most;
	u8 green_y_most;
	u8 blue_x_most;
	u8 blue_y_most;
	u8 white_point_x;
	u8 white_point_y;
	// Timings
	bits timings:24;
	u16 standard_timings[8];
	// Timing descriptor
	EdidDetailedTiming timing_detail[4];
	// Extension flag and checksum
	u8 num_exts;
	u8 checksum;
} Edid;
