// x86 System call handler.

.section .text

// Enter syscall via syscall/sysret extension.
.global sc_syscall
.align 0x10
sc_syscall:
	sti
	push %rcx
	push %r11
	call do_syscall
	pop %r11
	pop %rcx
	cli
	sysretq

// Pushes all relevant registers onto the stack, then passes a pointer as the first argument.
.global do_syscall
.align 0x10
.extern syscall_handler
do_syscall:
	swapgs /* Change TLS to kernel mode. */

	push	%rax
	push	%rdi
	push	%rsi
	push	%rdx
	push	%r10
	push	%r8
	push	%r9

	mov		%rsp, %rdi
	call	syscall_handler

	pop		%r9
	pop		%r8
	pop		%r10
	pop		%rdx
	pop		%rsi
	pop		%rdi
	pop		%rax

	swapgs /* Restore TLS to user mode. */
	ret
