/*? Interrupt definitions */

.intel_syntax noprefix

.global error_handler
.align 4
error_handler:
	call	interrupt_error
	iret

.global syscall_handler
.align 4
syscall_handler:
	call	interrupt_syscall
	iret
