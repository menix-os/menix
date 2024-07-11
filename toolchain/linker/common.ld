ENTRY(kernel_boot);

PHDRS
{
	headers	PT_LOAD		FILEHDR PHDRS;
	text	PT_LOAD		FLAGS(5);
	rodata	PT_LOAD		FLAGS(4);
	data	PT_LOAD		FLAGS(6);
}
