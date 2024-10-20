// x86-specific ELF constants.

#pragma once

#define EM_X86_64 62

#define R_X86_64_NONE	   0	 //	None	None
#define R_X86_64_64		   1	 //	qword	S + A
#define R_X86_64_PC32	   2	 //	dword	S + A – P
#define R_X86_64_GOT32	   3	 //	dword	G + A
#define R_X86_64_PLT32	   4	 //	dword	L + A – P
#define R_X86_64_COPY	   5	 //	None	Value is copied directly from shared object
#define R_X86_64_GLOB_DAT  6	 //	qword	S
#define R_X86_64_JUMP_SLOT 7	 //	qword	S
#define R_X86_64_RELATIVE  8	 //	qword	B + A
#define R_X86_64_GOTPCREL  9	 //	dword	G + GOT + A – P
#define R_X86_64_32		   10	 //	dword	S + A
#define R_X86_64_32S	   11	 //	dword	S + A
#define R_X86_64_16		   12	 //	word	S + A
#define R_X86_64_PC16	   13	 //	word	S + A – P
#define R_X86_64_8		   14	 //	word8	S + A
#define R_X86_64_PC8	   15	 //	word8	S + A – P
#define R_X86_64_PC64	   24	 //	qword	S + A – P
#define R_X86_64_GOTOFF64  25	 //	qword	S + A – GOT
#define R_X86_64_GOTPC32   26	 //	dword	GOT + A – P
#define R_X86_64_SIZE32	   32	 //	dword	Z + A
#define R_X86_64_SIZE64	   33	 //	qword	Z + A

#define EI_ARCH_CLASS	ELFCLASS64
#define EI_ARCH_DATA	ELFDATA2LSB
#define EI_ARCH_MACHINE EM_X86_64
