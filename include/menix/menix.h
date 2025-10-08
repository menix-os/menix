#ifndef _MENIX_MENIX_H
#define _MENIX_MENIX_H

#include <menix/archctl.h>
#include <menix/compiler.h>
#include <menix/rights.h>
#include <menix/status.h>
#include <menix/syscall_numbers.h>
#include <menix/syscall_stubs.h>
#include <menix/types.h>
#include <stddef.h>

__MENIX_CDECL_START

// Panics and returns an error status to the parent process.
[[noreturn]]
static inline void menix_panic(menix_status_t status) {
    menix_syscall1(MENIX_SYSCALL_PANIC, (menix_arg_t)status);
    unreachable();
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
    menix_handle_t* handles_buffer,
    size_t num_handles,
    void* data_buffer,
    size_t num_bytes
) {
    return menix_syscall5(
        MENIX_SYSCALL_PORT_CONNECT,
        (menix_arg_t)port,
        (menix_arg_t)handles_buffer,
        (menix_arg_t)num_handles,
        (menix_arg_t)data_buffer,
        (menix_arg_t)num_bytes
    );
}

static inline menix_status_t menix_port_write(menix_handle_t port) {
    return menix_syscall1(MENIX_SYSCALL_PORT_WRITE, (menix_arg_t)port);
}

static inline menix_status_t menix_port_read(menix_handle_t port) {
    return menix_syscall1(MENIX_SYSCALL_PORT_READ, (menix_arg_t)port);
}

__MENIX_CDECL_END

#endif
