/*? x86 System call handler. */

.section .text

/* Pushes all relevant registers onto the stack, then passes a pointer as the first argument. */
.global int_syscall_handler
.extern syscall_handler
.align 0x10
int_syscall_handler:
	sti
	push	%rax
	push	%rbx
	push	%rcx
	push	%rdx
	push	%rbp
	push	%rdi
	push	%rsi
	push	%r8
	push	%r9
	push	%r10
	push	%r11
	push	%r12
	push	%r13
	push	%r14
	push	%r15

	mov		%rsp, %rdi
	call	syscall_handler

	pop		%r15
	pop		%r14
	pop		%r13
	pop		%r12
	pop		%r11
	pop		%r10
	pop		%r9
	pop		%r8
	pop		%rsi
	pop		%rdi
	pop		%rbp
	pop		%rdx
	pop		%rcx
	pop		%rbx
	pop		%rax
	cli
	iretq
