.set MB_PAGE_ALIGN,	1<<0
.set MB_MEM_INFO,	1<<1
.set MB_VID_MODE, 	1<<2
.set MB_MAGIC,		0x1BADB002
.set MB_FLAGS,		MB_PAGE_ALIGN | MB_MEM_INFO
.set MB_CHECKSUM, 	-(MB_MAGIC + MB_FLAGS)

.section .multiboot
.align 4
.long MB_MAGIC
.long MB_FLAGS
.long MB_CHECKSUM

.section .bss
.align 16
stack_bottom:
.skip 16384
stack_top:

.section .text
.global _start
.type _start, @function
_start:
	mov $stack_top, %esp
	call kernel_main
	
	cli
1:	hlt
	jmp 1b
	
.size _start, . - _start
