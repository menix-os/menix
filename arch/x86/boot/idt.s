/*? Sets the interrupt descriptor table */

.intel_syntax noprefix
.code32

/* Temporary storage to hold the IDTR */
.align 16
idtr:
	.short	0
	.long	0

.global idt_set
.align 4
idt_set:
	mov		ax, [esp + 4]
	mov		[idtr], ax
	mov		eax, [esp + 8]
	mov		[idtr + 2], eax
	lidt	[idtr]
	ret

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
