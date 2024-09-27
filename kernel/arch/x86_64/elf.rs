pub const EM_X86_64: usize = 62;

//	None	None
pub const R_X86_64_NONE: usize = 0;
//	qword	S + A
pub const R_X86_64_64: usize = 1;
//	dword	S + A – P
pub const R_X86_64_PC32: usize = 2;
//	dword	G + A
pub const R_X86_64_GOT32: usize = 3;
//	dword	L + A – P
pub const R_X86_64_PLT32: usize = 4;
//	None	Value is copied directly from shared object
pub const R_X86_64_COPY: usize = 5;
//	qword	S
pub const R_X86_64_GLOB_DAT: usize = 6;
//	qword	S
pub const R_X86_64_JUMP_SLOT: usize = 7;
//	qword	B + A
pub const R_X86_64_RELATIVE: usize = 8;
//	dword	G + GOT + A – P
pub const R_X86_64_GOTPCREL: usize = 9;
//	dword	S + A
pub const R_X86_64_32: usize = 10;
//	dword	S + A
pub const R_X86_64_32S: usize = 11;
//	word	S + A
pub const R_X86_64_16: usize = 12;
//	word	S + A – P
pub const R_X86_64_PC16: usize = 13;
//	word8	S + A
pub const R_X86_64_8: usize = 14;
//	word8	S + A – P
pub const R_X86_64_PC8: usize = 15;
//	qword	S + A – P
pub const R_X86_64_PC64: usize = 24;
//	qword	S + A – GOT
pub const R_X86_64_GOTOFF64: usize = 25;
//	dword	GOT + A – P
pub const R_X86_64_GOTPC32: usize = 26;
//	dword	Z + A
pub const R_X86_64_SIZE32: usize = 32;
//	qword	Z + A
pub const R_X86_64_SIZE64: usize = 33;

// TODO
pub const EI_ARCH_CLASS: usize = ELFCLASS64;
pub const EI_ARCH_DATA: usize = ELFDATA2LSB;
pub const EI_ARCH_MACHINE: usize = EM_X86_64;
