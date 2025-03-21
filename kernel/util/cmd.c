#include <menix/memory/alloc.h>
#include <menix/util/cmd.h>

#include <string.h>

#define CMD_MAX 256

i8 atoi8(const char* num, u32 base);
i16 atoi16(const char* num, u32 base);
i32 atoi32(const char* num, u32 base);
i64 atoi64(const char* num, u32 base);

u8 atou8(const char* num, u32 base);
u16 atou16(const char* num, u32 base);
u32 atou32(const char* num, u32 base);
u64 atou64(const char* num, u32 base);

char* i8toa(i8 num, char* str, u32 base);
char* i16toa(i16 num, char* str, u32 base);
char* i32toa(i32 num, char* str, u32 base);
char* i64toa(i64 num, char* str, u32 base);

char* u8toa(u8 num, char* str, u32 base);
char* u16toa(u16 num, char* str, u32 base);
char* u32toa(u32 num, char* str, u32 base);
char* u64toa(u64 num, char* str, u32 base);

static const char* command_line = NULL;

void cmd_early_init(const char* data)
{
	command_line = data;
}

void cmd_init()
{
	// Save the command line to our own buffer.
	command_line = strdup(command_line);
}

// Returns the substring of the value part of the option specified by `key`.
// If not found, returns `NULL`.
static const char* cmd_parse(const char* key)
{
	// No argument given.
	if (key == NULL)
		return NULL;

	const usize key_len = strlen(key);
	const char* start = command_line;
	for (; *start != '\0'; start++)
	{
		if (strncmp(start, key, key_len) == 0)
		{
			// Options must be formatted as `key=value`. If they aren't, ignore them.
			if (start[key_len] == '=')
				return start + key_len + 1;
		}
	}

	// Not found.
	return NULL;
}

char* cmd_get_str(const char* key, const char* fallback)
{
	const char* val = cmd_parse(key);
	if (val != NULL)
	{
		char closing_char = ' ';
		usize value_length = 0;
		if (val[0] == '\"')
		{
			closing_char = '\"';
			val++;
		}
		for (; value_length < MIN(strlen(val), CMDLINE_MAX_LENGTH); value_length++)
			if (val[value_length] == closing_char)
				break;

		char* result = kzalloc(value_length + 1);
		memcpy(result, val, value_length);
		return result;
	}

	char* result = kzalloc(strlen(fallback) + 1);
	memcpy(result, fallback, strlen(fallback));
	return result;
}

usize cmd_get_usize(const char* key, usize fallback)
{
	const char* val = cmd_parse(key);
	if (val != NULL)
	{
		char closing_char = ' ';
		usize value_length = 0;
		for (; value_length < MIN(strlen(val), CMDLINE_MAX_LENGTH); value_length++)
			if (val[value_length] == closing_char)
				break;

		char buffer[CMD_MAX];
		const char* string = buffer;
		memcpy(buffer, val, MIN(value_length, sizeof(buffer)));
		buffer[value_length] = 0;

		// Parse the number.
		usize result;
		usize base = 10;
		if (strncmp(buffer, "0x", 2) == 0)
		{
			base = 16;
			string += 2;
		}

#if ARCH_BITS == 64
		result = atou64(string, base);
#else
		result = atou32(string, base);
#endif
		return result;
	}

	return fallback;
}

isize cmd_get_isize(const char* key, isize fallback)
{
	const char* val = cmd_parse(key);
	if (val != NULL)
	{
		char closing_char = ' ';
		usize value_length = 0;
		for (; value_length < MIN(strlen(val), CMDLINE_MAX_LENGTH); value_length++)
			if (val[value_length] == closing_char)
				break;

		char buffer[CMD_MAX];
		const char* string = buffer;
		memcpy(buffer, val, MIN(value_length, sizeof(buffer)));
		buffer[value_length] = 0;

		// Parse the number.
		isize result;
		const usize base = 10;

#if ARCH_BITS == 64
		result = atoi64(string, base);
#else
		result = atoi32(string, base);
#endif
		return result;
	}

	return fallback;
}
