#ifndef _MENIX_MEM_VM_H
#define _MENIX_MEM_VM_H

enum vm_flags {
    VM_READ = 1 << 0,
    VM_WRITE = 1 << 1,
    VM_EXEC = 1 << 2,
    VM_SHARED = 1 << 3,
};

#endif
