#ifndef __MENIX_SYSCALLS_H
#define __MENIX_SYSCALLS_H

#include <menix/archctl.h>
#include <menix/rights.h>
#include <menix/status.h>
#include <menix/syscall_numbers.h>
#include <menix/syscall_stubs.h>
#include <menix/types.h>
#include <stddef.h>

// Panics and returns an error status to the parent process.
static inline void menix_panic(menix_status_t status) {
    menix_syscall1(MENIX_SYSCALL_PANIC, (menix_arg_t)status);
}

static inline void menix_log(const char* message, size_t length) {
    menix_syscall2(MENIX_SYSCALL_LOG, (menix_arg_t)message, (size_t)length);
}

// Performs an architecture-dependent operation identified by `op`.
static inline menix_status_t menix_archctl(menix_archctl_t op, size_t value) {
    return menix_syscall2(MENIX_SYSCALL_ARCHCTL, (menix_arg_t)op, (menix_arg_t)value);
}

// Checks an object handle for validity.
static inline menix_status_t menix_handle_validate(menix_handle_t object) {
    return menix_syscall1(MENIX_SYSCALL_HANDLE_VALIDATE, (menix_arg_t)object);
}

// Drops a an object handle.
// All further references using this handle are invalid. The numerical value
// may become valid again, but it is an error to keep using it.
static inline menix_status_t menix_handle_drop(menix_handle_t handle) {
    return menix_syscall1(MENIX_SYSCALL_HANDLE_DROP, (menix_arg_t)handle);
}

// Clones an object handle.
static inline menix_status_t menix_handle_clone(
    menix_handle_t object,
    menix_rights_t cloned_rights,
    menix_handle_t* cloned
) {
    return menix_syscall2(MENIX_SYSCALL_HANDLE_CLONE, (menix_arg_t)object, (menix_arg_t)cloned);
}

static inline menix_status_t menix_mem_alloc(size_t length, menix_handle_t* out) {
    return menix_syscall2(MENIX_SYSCALL_MEM_ALLOC, (menix_arg_t)length, (menix_arg_t)out);
}

// Creates a new port.
static inline menix_status_t menix_port_create(
    enum menix_port_flags flags,
    menix_handle_t* endpoint0,
    menix_handle_t* endpoint1
) {
    return menix_syscall3(
        MENIX_SYSCALL_PORT_CREATE,
        (menix_arg_t)flags,
        (menix_arg_t)endpoint0,
        (menix_arg_t)endpoint1
    );
}

// Maps the message buffer in the address space and returns its base address.
// There may only be one message buffer per thread.
static inline menix_status_t menix_port_connect(
    menix_handle_t port,
    size_t num_handles,
    size_t num_bytes,
    menix_handle_t** out_handle_buf,
    void** out_data_buf
) {
    return menix_syscall5(
        MENIX_SYSCALL_PORT_CONNECT,
        (menix_arg_t)port,
        (menix_arg_t)num_handles,
        (menix_arg_t)num_bytes,
        (menix_arg_t)out_handle_buf,
        (menix_arg_t)out_data_buf
    );
}

static inline menix_status_t menix_port_action(menix_handle_t port, menix_port_action_t action) {
    return menix_syscall2(MENIX_SYSCALL_PORT_ACTION, (menix_arg_t)port, (menix_arg_t)action);
}

static inline menix_status_t menix_port_write(menix_handle_t port) {
    return menix_syscall1(MENIX_SYSCALL_PORT_ACTION, (menix_arg_t)port);
}

#endif
