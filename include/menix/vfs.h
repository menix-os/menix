#pragma once

#include <menix/errno.h>
#include <uapi/time.h>
#include <uapi/types.h>
#include <kernel/mem.h>
#include <kernel/spin.h>
#include <kernel/types.h>
#include <kernel/uio.h>
#include <stddef.h>
#include <stdint.h>

struct file;
struct inode;
struct path;

struct file_ops {
    menix_errno_t (*open)(struct file* self, uint32_t flags);
    menix_errno_t (*close)(struct file* self);
    menix_errno_t (*read)(struct file* self, struct iovec_iter* iter, ssize_t* out_read);
    menix_errno_t (*write)(struct file* self, struct iovec_iter* iter, ssize_t* out_written);
    menix_errno_t (*devctl)(struct file* self, uint32_t dcmd, void* data, size_t num, int* out_info);
    menix_errno_t (*poll)(struct file* self, int16_t mask, int16_t* out_mask); // TODO: This is not correct.
    menix_errno_t (*mmap)(
        struct file* self,
        struct address_space* space,
        virt_t addr,
        size_t len,
        int prot,
        int flags,
        off_t offset
    );
};

struct file {
    const struct file_ops* ops;
    struct inode* inode;
    uint32_t mode;
    uint32_t flags;
};

struct file* file_alloc();
struct file* file_from_fd(int fd);

struct inode_ops {
    union {
        struct {
            menix_errno_t (*lookup)(struct inode* inode, struct path* path);
        } dir;
        struct {
            menix_errno_t (*truncate)(struct inode* inode);
        } reg;
    };
};

struct inode_attr {
    struct timespec atime, mtime, ctime;
    uid_t uid;
    gid_t gid;
};

struct inode {
    const struct inode_ops* ops;
    struct inode_attr attr;
};

struct entry {
    const char* name;
    struct inode* inode;
    uint8_t state;
    struct spinlock lock;
};

struct path {
    struct entry* entry;
    struct mount* mount;
};

struct mount {
    uint32_t flags;
    struct entry root;
    struct path mount_point;
};

struct file_system_ops {};

struct file_system {
    const char* name;
    struct file_system_ops ops;
};
