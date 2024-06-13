/*------------------------------
Sets the global descriptor table
------------------------------*/

.intel_syntax noprefix

.align 16
gdtr:
	.short	0
	.long	0

.global gdt_set
gdt_set:
	cli
	mov		ax, [esp + 4] 	# Limit
	mov		[gdtr], ax
	mov		eax, [esp + 8]	# Base
	mov		[gdtr + 2], eax
	lgdt 	[gdtr]
	mov		ax, 0x10
	mov		ds, ax
	mov		es, ax
	mov		fs, ax
	mov		gs, ax
	mov		ss, ax
	jmp		0x08:flush
flush:
	ret
