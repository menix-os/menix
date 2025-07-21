#ifndef _MENIX_USER_H
#define _MENIX_USER_H

#include <menix/types.h>
#include <menix/hint.h>

void copy_from_user(u8* dst, const u8 __user* src, usize num);
void copy_to_user(u8 __user* dst, const u8* src, usize num);

#endif
