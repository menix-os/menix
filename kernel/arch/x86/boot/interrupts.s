/*? Interrupt definitions */

.section text

.global int_error_handler
.align 0x10
int_error_handler:
	call	interrupt_error
	iret

.global int_syscall_handler
.align 0x10
int_syscall_handler:
	swapgs

	push	0x1b
	push	%gs:0016
	push	%r11
	push	0x23
	push	%rcx

	sub		$24, %rsp

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
	call	interrupt_syscall

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

	add		$24, %rsp

	swapgs

	iret
