// Interrupt handling

.section .text

// Pushes all general purpose registers onto the stack.
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

// Pops all general purpose registers from the stack.
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

.align 0x10
.global arch_return_to_user
arch_return_to_user:
	cli						/* Disable interrupts. */
	movq	%rsp,	%gs:8	/* Save kernel stack to `Cpu.kernel_stack`. */
	movq	%gs:16,	%rsp	/* Load user stack from `Cpu.user_stack`. */
	movq	%rsp,	%rbp	/* Save stack base pointer. */
	movq	%rdi,	%rcx	/* rcx = Instruction pointer */
	mov		$0x202,	%r11	/* Set RFLAGS */
	swapgs					/* Change GS to user mode. */
	sti						/* Resume interrupts. */
	sysretq					/* Return to user mode */

// Enter syscall via AMD64 syscall/sysret instructions.
.global sc_syscall
.extern syscall_handler
.align 0x10
sc_syscall:
	cli							/* Disable interrupts. */
	swapgs						/* Change GS to kernel mode. */
	movq	%rsp,	%gs:16		/* Save user stack to `Cpu.user_stack`. */
	movq	%gs:8,	%rsp		/* Restore kernel stack from `Cpu.kernel_stack`. */
	cld							/* Clear direction bit from RFLAGS */
	/* We're pretending to be an interrupt, so fill the bottom fields of CpuRegisters. */
	/* For details see: https://www.felixcloutier.com/x86/syscall */
	pushq	$0x23				/* SS and CS are not changed during SYSCALL. Use `gdt_table.user_data & CPL_USER`. */
	pushq	%gs:16				/* Get RSP from when we saved it */
	pushq	%r11				/* RFLAGS is moved into r11 by the CPU. */
	pushq	$0x2b				/* Same as SS. Use `gdt_table.user_code64 & CPL_USER` */
	pushq	%rcx				/* RIP is moved into rcx by the CPU. */
	pushq	$0x00				/* CpuRegisters.error field */
	pushq	$0x00				/* CpuRegisters.isr field */
	pushq	$0x00				/* CpuRegisters.core field */
	push_all_regs				/* Push general purpose registers so they can be written to by syscalls */

	mov		%rsp,	%rdi		/* Put CpuRegisters* as first argument */
	call	syscall_handler		/* Call syscall handler */

	pop_all_regs				/* Pop stack values back to the general purpose registers. */
	add		$0x18,	%rsp		/* Skip .error, .isr and .core fields */
	movq	%gs:16,	%rsp		/* Load user stack from `Cpu.user_stack`. */
	swapgs						/* Change GS to user mode. */
	sti							/* Resume interrupts. */
	sysretq						/* Return to user mode */

// Swaps GSBASE if CPL == USER
.macro swapgs_if_necessary
	cmpw	$0x08,	0x8(%rsp)
	je		1f
	swapgs
1:
.endm

// Internal function called by one of the stubs.
.align 0x10
interrupt_internal:
	pushq	%gs					/* Push CPU ID. */
	push_all_regs
	mov		%rsp,	%rdi		/* Load the CpuRegisters* as first argument */
	xor		%rbp,	%rbp		/* Zero out the base pointer since we can't trust it */
	call	interrupt_handler	/* Call interrupt handler */
	pop_all_regs
	add		$0x18,	%rsp		/* Skip .error, .isr, and .core fields */
	swapgs_if_necessary			/* Change GS back to user mode if we came from user mode. */
	iretq

// Interrupt stub that pushes 0 as the error code.
.macro interrupt_stub num
.global interrupt_\num
.align 0x10
interrupt_\num:
	swapgs_if_necessary			/* Change GS to kernel mode if we're coming from user mode. */
	pushq	$0
	pushq	$\num
	jmp		interrupt_internal
.endm

// Interrupt stub with an actual error code.
.macro interrupt_stub_err num
.global interrupt_\num
.align 0x10
interrupt_\num:
	swapgs_if_necessary			/* Change GS to kernel mode if we're coming from user mode. */
	pushq	$\num
	jmp		interrupt_internal
.endm

// Define 256 interrupt stubs using the macros above.
.extern interrupt_handler
.altmacro
.set i, 0
.rept 256
.if (i == 8 || (i >= 10 && i <= 14) || i == 17 || i == 21 || i == 29 || i == 30)
	interrupt_stub_err %i
.else
	interrupt_stub %i
.endif
	.set i, i+1
.endr
