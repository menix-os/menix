// ELF parsing utilities.

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/elf.h>
#include <menix/util/log.h>

#include <string.h>

bool elf_load(PageMap* page_map, const void* address, usize length, ElfInfo* info)
{
	if (page_map == NULL)
	{
		print_error("elf: Failed to load ELF: No page map given!\n");
		return false;
	}

	// Read ELF header.
	Elf_Hdr hdr;
	memcpy(&hdr, address, sizeof(Elf_Hdr));

	// Verify header magic.
	if (memcmp(hdr.e_ident, ELF_MAG, sizeof(ELF_MAG)) != 0)
	{
		print_log("elf: Failed to load ELF: File is not an ELF executable.\n");
		return false;
	}

	// Verify ELF identification.
	if (hdr.e_ident[EI_CLASS] != EI_ARCH_CLASS || hdr.e_ident[EI_DATA] != EI_ARCH_DATA ||
		hdr.e_ident[EI_VERSION] != EV_CURRENT || hdr.e_ident[EI_OSABI] != ELFOSABI_SYSV ||
		hdr.e_machine != EI_ARCH_MACHINE || hdr.e_phentsize != sizeof(Elf_Phdr))
	{
		print_log("elf: Failed to load ELF: File is not designed to run on this machine.\n");
		return false;
	}

	// Evaluate program headers.
	Elf_Phdr phdr;
	for (usize i = 0; i < hdr.e_phnum; i++)
	{
		// Read the current header.
		const usize phdr_off = hdr.e_phoff + (i * sizeof(Elf_Phdr));
		memcpy(&phdr, address + phdr_off, sizeof(Elf_Phdr));

		switch (phdr.p_type)
		{
			case PT_LOAD:
			{
				// Calculate protetion
				VMProt prot = 0;
				if (phdr.p_flags & PF_R)
					prot |= VMProt_Read;
				else
					print_log("elf: Potential bug: Program header %zu does not have read permission.\n");
				if (phdr.p_flags & PF_W)
					prot |= VMProt_Write;
				if (phdr.p_flags & PF_X)
					prot |= VMProt_Execute;

				const usize page_size = vm_get_page_size(VMLevel_Small);

				// Align the virtual address for mapping.
				VirtAddr aligned_virt = ALIGN_DOWN(phdr.p_vaddr, page_size);
				usize align_difference = phdr.p_vaddr - aligned_virt;

				// Amount of pages to allocate for this segment.
				const usize page_count = ALIGN_UP(phdr.p_memsz + align_difference, page_size) / page_size;

				// Map the physical pages to the requested address.
				for (usize p = 0; p < page_count; p++)
				{
					PhysAddr page = pm_alloc(1);
					if (vm_map(page_map, page, (VirtAddr)(aligned_virt + (p * page_size)), prot, VMFlags_User,
							   VMLevel_Small) == false)
					{
						print_log("elf: Failed to load ELF: Could not map %zu pages to 0x%p.\n", page_count,
								  aligned_virt);
						// TODO: Undo previous maps.
						return false;
					}
				}

				// Create temporary mapping in the kernel page map so we can write to it.
				void* foreign = vm_map_foreign(page_map, aligned_virt, page_count);
				if (foreign == (void*)~0UL)
				{
					print_log("elf: Failed to load ELF: Could not map foreign memory region!\n");
					return false;
				}

				// Load data from file.
				memcpy(foreign + align_difference, address + phdr.p_offset, phdr.p_filesz);

				// Zero out the remaining data.
				memset(foreign + align_difference + phdr.p_filesz, 0, phdr.p_memsz - phdr.p_filesz);

				// Destroy temporary mapping.
				if (vm_unmap_foreign(foreign, page_count) == false)
				{
					print_log("elf: Failed to load ELF: Could not unmap foreign memory region!\n");
					return false;
				}

				break;
			}
			case PT_PHDR:
			{
				info->at_phdr = phdr.p_vaddr;
				break;
			}
			default:
			{
				print_error("elf: Failed to load ELF: Unsupported program header type %p!\n", phdr.p_type);
				return false;
			}
		}
	}

	info->at_entry = hdr.e_entry;
	info->at_phnum = hdr.e_phnum;
	info->at_phent = hdr.e_phentsize;

	return true;
}
