// Command line options and parsing

#pragma once
#include <menix/common.h>

typedef struct
{
	usize terminal;		// Output redirection. Possible values are: serial, fb, all
	char* icon_path;	// The file path to the splash boot icon displayed after boot.
} CmdOptions;

// Parses a semicolon-seperated list of command line arguments and writes the parsed result to the provided structure.
void cmd_parse(CmdOptions* target, const char* cmd_line);
