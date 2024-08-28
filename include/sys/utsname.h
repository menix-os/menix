// sys/utsname.h - system name structure

#ifndef __MENIX_SYS_UTSNAME_H__
#define __MENIX_SYS_UTSNAME_H__

#include <sys/types.h>

__MENIX_START_DECL

struct utsname
{
	char sysname[64];
	char nodename[64];
	char release[64];
	char version[64];
	char machine[64];
};

__MENIX_END_DECL

#endif
