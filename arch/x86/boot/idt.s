/*---------------------------------
Sets the interrupt descriptor table
---------------------------------*/

.intel_syntax noprefix

# Temporary storage to hold the IDTR
.align 16
idtr:
	.short	0
	.long	0

# void idt_set(limit, base)
.global idt_set
.align 4
idt_set:
	mov		ax, [esp + 4]
	mov		[idtr], ax
	mov		eax, [esp + 8]
	mov		[idtr + 2], eax
	lidt	[idtr]
	ret

.global io_in
.align 4
io_in:
	mov		edx, [esp + 4]
	in		al, dx
	ret

.global io_out
.align 4
io_out:
	mov		edx, [esp + 4]
	mov		eax, [esp + 8]
	out		dx, al
	ret

.global enable_interrupts
.align 4
enable_interrupts:
	sti
	ret

.global error_handler
.align 4
error_handler:
	call	interrupt_error
	iret
