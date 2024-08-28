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

// Swaps GSBASE if CPL == USER
.macro swapgs_if_necessary
	andb	$0x03,	0x24(%rsp)
	jz		1f
	swapgs
1:
.endm

// Internal function called by one of the stubs.
interrupt_internal:
	push_all_regs
	mov		%rsp,	%rdi
	call	interrupt_handler
	pop_all_regs
	add		$24,	%rsp
	swapgs_if_necessary
	iretq

// Interrupt stub that pushes 0 as the error code.
.macro interrupt_stub num
.global interrupt_\num
.align 0x10
interrupt_\num:
	swapgs_if_necessary
	pushq	$0
	pushq	$\num
	pushq	%fs
	jmp		interrupt_internal
.endm

// Interrupt stub with an actual error code.
.macro interrupt_stub_err num
.global interrupt_\num
.align 0x10
interrupt_\num:
	swapgs_if_necessary
	pushq	$\num
	pushq	%fs
	jmp		interrupt_internal
.endm

// Enter syscall via AMD64 syscall/sysret instructions.
.global sc_syscall
.extern syscall_handler
.align 0x10
sc_syscall:
	swapgs						/* Change GS to kernel mode. */
	cli							/* Disable interrupts. */
	movq	%rsp,	%gs:16		/* Save user stack to `Cpu.user_stack`. */
	movq	%gs:8,	%rsp		/* Restore kernel stack from `Cpu.kernel_stack`. */
	cld							/* Clear direction bit from RFLAGS */
	/* We're pretending to be an interrupt, so fill the bottom fields of CpuRegisters. */
	/* For details see: https://www.felixcloutier.com/x86/syscall */
	push	$0x23				/* SS and CS are not changed during SYSCALL. Use `gdt_table.user_data & CPL_USER`. */
	push	%r11				/* RFLAGS is moved into r11 by the CPU. */
	push	$0x2b				/* Same as SS. Use `gdt_table.user_code64 & CPL_USER` */
	push	%rcx				/* RIP is moved into rcx by the CPU. */
	sub		$0x24, %rsp			/* Skip .error, .isr and .core fields. */

	push_all_regs				/* Push general purpose registers so they can be passed to the syscall */
	mov		%rsp,	%rdi		/* Put CpuRegisters* as first argument */
	call	syscall_handler
	pop_all_regs				/* Pop stack values back to the general purpose registers. */
	add		$24,	%rsp		/* We don't want to taint the popped registers, so skip the stuff pushed by SYSCALL. */
	sti							/* Resume interrupts. */
	swapgs						/* Change GS to user mode. */
	sysretq						/* Return back to (q == 64-bit!) user space. */

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
