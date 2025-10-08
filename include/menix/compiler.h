#ifndef _MENIX_COMPILER_H
#define _MENIX_COMPILER_H

#ifdef __cplusplus
#define __MENIX_CDECL_START extern "C" {
#define __MENIX_CDECL_END   }
#else
#define __MENIX_CDECL_START
#define __MENIX_CDECL_END
#endif

#endif
