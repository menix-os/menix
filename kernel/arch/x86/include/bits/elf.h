// x86-specific ELF constants.

#define EM_X86_64 62

#define R_386_NONE	   0	 // None	None
#define R_386_32	   2	 // dword	S + A
#define R_386_PC32	   1	 // dword	S + A – P
#define R_386_GOT32	   3	 // dword	G + A
#define R_386_PLT32	   4	 // dword	L + A – P
#define R_386_COPY	   5	 // None	Value is copied directly from shared object
#define R_386_GLOB_DAT 6	 // dword	S
#define R_386_JMP_SLOT 7	 // dword	S
#define R_386_RELATIVE 8	 // dword	B + A
#define R_386_GOTOFF   9	 // dword	S + A – GOT
#define R_386_GOTPC	   10	 // dword	GOT + A – P
#define R_386_32PLT	   11	 // dword	L + A
#define R_386_16	   20	 // word	S + A
#define R_386_PC16	   21	 // word	S + A – P
#define R_386_8		   22	 // byte	S + A
#define R_386_PC8	   23	 // byte	S + A – P
#define R_386_SIZE32   38	 // dword	z + A

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
