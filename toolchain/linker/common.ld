ENTRY(kernel_boot);

PHDRS
{
	null	PT_LOAD		FILEHDR PHDRS;
	text	PT_LOAD		FLAGS(5);
	data	PT_LOAD		FLAGS(6);
	rodata	PT_LOAD		FLAGS(4);
}
