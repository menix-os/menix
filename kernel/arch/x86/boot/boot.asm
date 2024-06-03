/*-----------
x86 Bootstrap
-----------*/

# Stack
.section .bootstrap_stack, "aw", @nobits
stack_bottom:
.skip 32768 # 32 KiB
stack_top:

.section .bss, "aw", @nobits
	.align 4096
boot_page_directory:
	.skip 4096
boot_page_table1:
	.skip 4096

.section .text, "a"
.global _start
.type _start, @function
_start:
kernel_init:
	cli							# Disable interrupts.
	mov $stack_top, %esp		# Setup the stack.
	call kernel_main			# Call the kernel entry point.
	cli
kernel_hang:
	hlt							# Hang once kernel exits.
	jmp kernel_hang
