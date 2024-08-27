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

// Internal function called by one of the stubs.
interrupt_internal:
	push_all_regs
	mov %rsp, %rdi
	call interrupt_handler
	pop_all_regs
	add $24, %rsp
	iretq

// Interrupt stub that pushes 0 as the error code.
.macro interrupt_stub num
.global interrupt_\num
.align 0x10
interrupt_\num:
	pushq $0
	pushq $\num
	pushq %fs
	jmp interrupt_internal
.endm

// Interrupt stub with an actual error code.
.macro interrupt_stub_err num
.global interrupt_\num
.align 0x10
interrupt_\num:
	push_all_regs
	pushq $\num
	pushq %fs
	jmp interrupt_internal
.endm

// Enter syscall via software interrupt 0x80.
.global interrupt_syscall
.extern do_syscall
.align 0x10
interrupt_syscall:
	sti
	call do_syscall
	cli
	iretq

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
