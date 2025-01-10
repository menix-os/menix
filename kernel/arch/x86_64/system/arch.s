/* x86 architecture specific functions */

.section .text
.altmacro

/* Pushes all general purpose registers onto the stack. */
.macro push_all_regs
	pushq %rax
	pushq %rbx
	pushq %rcx
	pushq %rdx
	pushq %rbp
	pushq %rdi
	pushq %rsi
	pushq %r8
	pushq %r9
	pushq %r10
	pushq %r11
	pushq %r12
	pushq %r13
	pushq %r14
	pushq %r15
.endm

/* Pops all general purpose registers from the stack. */
.macro pop_all_regs
	popq %r15
	popq %r14
	popq %r13
	popq %r12
	popq %r11
	popq %r10
	popq %r9
	popq %r8
	popq %rsi
	popq %rdi
	popq %rbp
	popq %rdx
	popq %rcx
	popq %rbx
	popq %rax
.endm

/* Enter syscall via AMD64 syscall/sysret instructions. */
.global arch_syscall_internal
.align 0x10
arch_syscall_internal:
	swapgs
	movq	%rsp,	%gs:16		/* Save user stack to `Cpu.user_stack`. */
	movq	%gs:8,	%rsp		/* Restore kernel stack from `Cpu.kernel_stack`. */
	cld
	/* We're pretending to be an interrupt, so fill the bottom fields of CpuRegisters. */
	/* For details see: https://www.felixcloutier.com/x86/syscall */
	pushq	$0x23				/* SS and CS are not changed during SYSCALL. Use `gdt_table.user_data | CPL_USER`. */
	pushq	%gs:16				/* Get RSP from when we saved it */
	pushq	%r11				/* RFLAGS is moved into r11 by the CPU. */
	pushq	$0x2b				/* Same as SS. Use `gdt_table.user_code64 | CPL_USER` */
	pushq	%rcx				/* RIP is moved into rcx by the CPU. */
	pushq	$0x00				/* Context.error field */
	push_all_regs
	mov		$0x80,	%rdi		/* 0x80 is the ISR for syscall. It's not used here, but who knows... */
	mov		%rsp,	%rsi		/* Load the Context* as second argument. */
	xor		%rbp,	%rbp
	.extern syscall_handler
	call	syscall_handler
	mov		%rax,	%rsp
	pop_all_regs
	add		$0x8,	%rsp		/* Skip Context.error field. */
	movq	%gs:16,	%rsp		/* Load user stack from `Cpu.user_stack`. */
	swapgs
	sysretq

/* Swaps GSBASE if CPL == KERNEL */
.macro swapgs_if_necessary
	cmpw	$0x8,	0x10(%rsp)
	je		1f
	swapgs
1:
.endm

/* Define 256 interrupt stubs using the macro above. */
.rept 256
.align 0x10
arch_int_\+:
.if !(\+ == 8 || (\+ >= 10 && \+ <= 14) || \+ == 17 || \+ == 21 || \+ == 29 || \+ == 30)
	pushq	$0					/* If this is an interrupt that doesn't push an error code, push one ourselves. */
.endif
	swapgs_if_necessary
	push_all_regs
	mov		$\+,	%rdi		/* Load the ISR as first argument */
	mov		%rsp,	%rsi		/* Load the Context* as second argument. */
	xor		%rbp,	%rbp
	.extern int_handler
	call	int_handler
	mov		%rax,	%rsp		/* int_handler returns a pointer to the new context. */
	pop_all_regs
	swapgs_if_necessary
	add		$0x8,	%rsp		/* Skip Context.error field. */
	iretq
.endr

/* Build a table of all the interrupt stubs */
.section .rodata
.global arch_int_table
arch_int_table:
.rept 256
	.quad arch_int_\+
.endr
