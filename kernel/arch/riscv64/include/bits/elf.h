// riscv64-specific ELF constants.

#pragma once

#define EM_RISCV 243

#define R_RISCV_NONE			  0		//
#define R_RISCV_32				  1		// _word32_  S + A
#define R_RISCV_64				  2		// _word64_  S + A
#define R_RISCV_RELATIVE		  3		// _wordclass_ B + A
#define R_RISCV_COPY			  4		//
#define R_RISCV_JUMP_SLOT		  5		// _wordclass_ S
#define R_RISCV_TLS_DTPMOD32	  6		// _word32_ TLSMODULE
#define R_RISCV_TLS_DTPMOD64	  7		// _word64_ TLSMODULE
#define R_RISCV_TLS_DTPREL32	  8		// _word32_ S + A - TLS_DTV_OFFSET
#define R_RISCV_TLS_DTPREL64	  9		// _word64_ S + A - TLS_DTV_OFFSET
#define R_RISCV_TLS_TPREL32		  10	// _word32_ S + A + TLSOFFSET
#define R_RISCV_TLS_TPREL64		  11	// _word64_ S + A + TLSOFFSET
#define R_RISCV_TLSDESC			  12	// TLSDESC(S+A)
#define R_RISCV_BRANCH			  16	// _B-Type_ S + A - P
#define R_RISCV_JAL				  17	// _J-Type_ S + A - P
#define R_RISCV_CALL			  18	// _U+I-Type_ S + A - P
#define R_RISCV_CALL_PLT		  19	// _U+I-Type_ S + A - P
#define R_RISCV_GOT_HI20		  20	// _U-Type_ G + GOT + A - P
#define R_RISCV_TLS_GOT_HI20	  21	// _U-Type_
#define R_RISCV_TLS_GD_HI20		  22	// _U-Type_
#define R_RISCV_PCREL_HI20		  23	// _U-Type_ S + A - P
#define R_RISCV_PCREL_LO12_I	  24	// _I-type_ S - P
#define R_RISCV_PCREL_LO12_S	  25	// _S-Type_ S - P
#define R_RISCV_HI20			  26	// _U-Type_ S + A
#define R_RISCV_LO12_I			  27	// _I-Type_ S + A
#define R_RISCV_LO12_S			  28	// _S-Type_ S + A
#define R_RISCV_TPREL_HI20		  29	// _U-Type_
#define R_RISCV_TPREL_LO12_I	  30	// _I-Type_
#define R_RISCV_TPREL_LO12_S	  31	// _S-Type_
#define R_RISCV_TPREL_ADD		  32	//
#define R_RISCV_ADD8			  33	// _word8_ V + S + A
#define R_RISCV_ADD16			  34	// _word16_ V + S + A
#define R_RISCV_ADD32			  35	// _word32_ V + S + A
#define R_RISCV_ADD64			  36	// _word64_ V + S + A
#define R_RISCV_SUB8			  37	// _word8_ V - S - A
#define R_RISCV_SUB16			  38	// _word16_ V - S - A
#define R_RISCV_SUB32			  39	// _word32_ V - S - A
#define R_RISCV_SUB64			  40	// _word64_ V - S - A
#define R_RISCV_GOT32_PCREL		  41	// _word32_ G + GOT + A - P
#define R_RISCV_ALIGN			  43	//
#define R_RISCV_RVC_BRANCH		  44	// _CB-Type_ S + A - P
#define R_RISCV_RVC_JUMP		  45	// _CJ-Type_ S + A - P
#define R_RISCV_RELAX			  51	//
#define R_RISCV_SUB6			  52	// _word6_ V - S - A
#define R_RISCV_SET6			  53	// _word6_ S + A
#define R_RISCV_SET8			  54	// _word8_ S + A
#define R_RISCV_SET16			  55	// _word16_ S + A
#define R_RISCV_SET32			  56	// _word32_ S + A
#define R_RISCV_32_PCREL		  57	// _word32_ S + A - P
#define R_RISCV_IRELATIVE		  58	// _wordclass_ `ifunc_resolver(B + A)`
#define R_RISCV_PLT32			  59	// _word32_ S + A - P
#define R_RISCV_SET_ULEB128		  60	// _ULEB128_ S + A
#define R_RISCV_SUB_ULEB128		  61	// _ULEB128_ V - S - A
#define R_RISCV_TLSDESC_HI20	  62	// _U-Type_ S + A - P
#define R_RISCV_TLSDESC_LOAD_LO12 63	// _I-Type_ S - P
#define R_RISCV_TLSDESC_ADD_LO12  64	// _I-Type_ S - P
#define R_RISCV_TLSDESC_CALL	  65	//

#define EI_ARCH_CLASS	ELFCLASS64
#define EI_ARCH_DATA	ELFDATA2LSB
#define EI_ARCH_MACHINE EM_RISCV
