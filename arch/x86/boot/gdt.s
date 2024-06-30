/*? Sets the global descriptor table */

.intel_syntax noprefix
.code32

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
.att_syntax prefix			# Workaround because LLVM has a parser bug at the time of writing this.
	# ljmp	$0x08, $flush
.intel_syntax noprefix
flush:
	mov		ax, 0x10
	mov		ds, ax
	mov		es, ax
	mov		fs, ax
	mov		gs, ax
	mov		ss, ax
	ret
