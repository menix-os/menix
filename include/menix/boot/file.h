#ifndef _MENIX_BOOT_FILE_H
#define _MENIX_BOOT_FILE_H

#include <stddef.h>
#include <stdint.h>

struct boot_file {
    uint8_t* data;
    size_t length;
    const char* path;
};

extern struct boot_file boot_files[32];
extern size_t boot_files_count;

#endif
