/* x86 architecture specific functions */

.section .text
.altmacro

/* Swaps GSBASE if CPL == KERNEL */
.macro swapgs_if_necessary
	cmpw	$0x8,	0x8(%rsp)
	je		1f
	swapgs
1:
.endm

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
.global sc_syscall
.extern syscall_handler
.align 0x10
sc_syscall:
	swapgs						/* Change GS to kernel mode. */
	movq	%rsp,	%gs:16		/* Save user stack to `Cpu.user_stack`. */
	movq	%gs:8,	%rsp		/* Restore kernel stack from `Cpu.kernel_stack`. */
	cld							/* Clear direction bit from RFLAGS */
	/* We're pretending to be an interrupt, so fill the bottom fields of CpuRegisters. */
	/* For details see: https://www.felixcloutier.com/x86/syscall */
	pushq	$0x23				/* SS and CS are not changed during SYSCALL. Use `gdt_table.user_data | CPL_USER`. */
	pushq	%gs:16				/* Get RSP from when we saved it */
	pushq	%r11				/* RFLAGS is moved into r11 by the CPU. */
	pushq	$0x2b				/* Same as SS. Use `gdt_table.user_code64 | CPL_USER` */
	pushq	%rcx				/* RIP is moved into rcx by the CPU. */
	pushq	$0x00				/* Context.error field */
	pushq	$0x00				/* Context.isr field */
	push_all_regs				/* Push general purpose registers so they can be written to by syscalls */
	mov		%rsp,	%rdi		/* Load the Context* as first argument. */
	xor		%rbp,	%rbp		/* Zero out the base pointer. */
	call	syscall_handler		/* Call syscall handler */
	pop_all_regs				/* Pop stack values back to the general purpose registers. */
	add		$0x10,	%rsp		/* Skip Context.error and Context.isr fields. */
	movq	%gs:16,	%rsp		/* Load user stack from `Cpu.user_stack`. */
	swapgs						/* Change GS to user mode. */
	sysretq						/* Return to user mode */

/* Internal function called by one of the stubs. */
.align 0x10
interrupt_internal:
	push_all_regs
	mov		%rsp,	%rdi		/* Load the Context* as first argument. */
	xor		%rbp,	%rbp		/* Zero out the base pointer so we don't backtrace into the user program */
	.extern isr_handler
	call	isr_handler			/* Call interrupt handler */
	mov		%rax,	%rsp		/* interrupt_handler returns a pointer to the new context. */
	pop_all_regs
	add		$0x10,	%rsp		/* Skip Context.error and Context.isr fields. */
	swapgs_if_necessary			/* Change GS back to user mode if we came from user mode. */
	iretq

/* Define 256 interrupt stubs using the macro above. */
.rept 256
.align 0x10
interrupt_\+:
	swapgs_if_necessary			/* Change GS to kernel mode if we're coming from user mode. */
.if !(\+ == 8 || (\+ >= 10 && \+ <= 14) || \+ == 17 || \+ == 21 || \+ == 29 || \+ == 30)
	pushq	$0					/* If this is an interrupt that doesn't push an error code, push one ourselves. */
.endif
	pushq	$\+					/* Push the ISR to the stack. */
	jmp		interrupt_internal
.endr

/* Build a table of all the interrupt stubs */
.section .rodata
.global interrupt_table
interrupt_table:
.rept 256
	.quad interrupt_\+
.endr
