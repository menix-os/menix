/*---------------------------------
Sets the interrupt descriptor table
---------------------------------*/

# Temporary storage to hold the IDTR
.align 16
idtr:
	.short	0
	.long	0

# void idt_set(limit, base)
.global idt_set
.align 4
idt_set:
	mov   +4(%esp), %ax
	mov   %ax, [idtr]
	mov   +8(%esp), %eax
	mov   %eax, [idtr + 2]
	lidt  [idtr]
	ret

.global io_in
.align 4
io_in:
	mov +4(%esp), %edx
	in %dx, %al
	ret

.global io_out
.align 4
io_out:
	mov +4(%esp), %edx
	mov +8(%esp), %eax
	out %al, %dx
	ret

.global enable_interrupts
.align 4
enable_interrupts:
	sti
	ret

.global error_handler
.align 4
error_handler:
	call interrupt_error
	iret
