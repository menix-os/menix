#ifndef _MENIX_SYSCALLS_H
#define _MENIX_SYSCALLS_H

#include <menix/action.h>
#include <menix/object.h>
#include <menix/status.h>
#include <stdint.h>

[[noreturn]]
void menix_panic();

menix_status_t menix_object_check(menix_object_t object);
menix_status_t menix_object_close(menix_object_t object);
menix_status_t menix_object_clone(menix_object_t object, menix_object_t* out);

menix_status_t menix_link_create(uint32_t options, menix_object_t* out0, menix_object_t* out1);
// menix_link_read
// menix_link_write

menix_status_t menix_action_register(uint32_t index, menix_action_t action, menix_object_t* out);
menix_status_t menix_action_unregister(menix_object_t object);
menix_status_t menix_action_await();

#endif
