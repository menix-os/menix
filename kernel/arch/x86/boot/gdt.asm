/*------------------------------
Sets the global descriptor table
------------------------------*/

.align 16
gdtr:
	.short	0
	.long	0

.global gdt_set
gdt_set:
	cli
	movw	4(%esp), %ax 	# Limit
	movw	%ax, [gdtr]
	movl	8(%esp), %eax	# Base
	movl	%eax, [gdtr + 2]
	lgdt 	[gdtr]
	movw	$0x10, %ax
	movw	%ax, %ds
	movw	%ax, %es
	movw	%ax, %fs
	movw	%ax, %gs
	movw	%ax, %ss
	jmp		$0x08,$flush
flush:
	ret
