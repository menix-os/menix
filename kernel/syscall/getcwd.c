#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>

SYSCALL_IMPL(getcwd, VirtAddr buf, usize size)
{
	if (buf == 0 || size == 0)
		return SYSCALL_ERR(EINVAL);
	if (size > PATH_MAX)
		return SYSCALL_ERR(ERANGE);

	Process* proc = arch_current_cpu()->thread->parent;

	// Get the path.
	char* kernel_buf = kmalloc(PATH_MAX);
	usize written = vfs_get_path(proc->working_dir, kernel_buf, size);

	// Copy the result to the user.
	vm_user_write(proc, buf, kernel_buf, written);
	kfree(kernel_buf);

	return SYSCALL_OK(0);
}
