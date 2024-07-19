//? x86 System call handler.

.section .text

// Enter syscall via software interrupt 0x80.
.global int_syscall_handler
.align 0x10
int_syscall_handler:
	sti
	call do_syscall
	cli
	iretq

// Enter syscall via syscall/sysret extension.
.global sc_syscall_handler
.align 0x10
sc_syscall_handler:
// TODO: Save RSP as it gets lost if we got here via syscall instruction!
	call do_syscall
	sysret

// Pushes all relevant registers onto the stack, then passes a pointer as the first argument.
.global do_syscall
.extern syscall_handler
.align 0x10
do_syscall:
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
	ret
