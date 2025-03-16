use core::sync::atomic::{AtomicUsize, Ordering};

use super::thread::Thread;
use crate::generic::{
    alloc::phys,
    alloc::virt::{PageTable, VmFlags},
    elf::{self, ElfHdr, ElfPhdr},
    errno::Errno,
};
use alloc::vec::Vec;

pub struct Process {
    id: usize,
    is_user: bool,
    page_table: PageTable,
    threads: Vec<Thread>,
}

impl Process {
    pub fn new(is_user: bool) -> Self {
        Self {
            id: PID_COUNTER.fetch_add(1, Ordering::Relaxed),
            is_user,
            page_table: PageTable::new(is_user),
            threads: Vec::new(),
        }
    }

    /// Loads an ELF executable from memory into a new Process.
    pub fn from_elf(is_user: bool, data: &[u8]) -> Result<Self, Errno> {
        let elf_hdr: &ElfHdr = bytemuck::try_from_bytes(&data[0..size_of::<ElfHdr>()])
            .expect("Couldn't read the ELF header");

        // Check ELF magic.
        if elf_hdr.e_ident[0..4] != elf::ELF_MAG {
            return Err(Errno::EINVAL);
        }

        // We only support 64-bit.
        if elf_hdr.e_ident[elf::EI_CLASS] != elf::ELFCLASS64 {
            return Err(Errno::EINVAL);
        }

        // Check endianness.
        #[cfg(target_endian = "little")]
        if elf_hdr.e_ident[elf::EI_DATA] != elf::ELFDATA2LSB {
            return Err(Errno::EINVAL);
        }
        #[cfg(target_endian = "big")]
        if elf_hdr.e_ident[EI_DATA] != ELFDATA2MSB {
            return Err(Errno::EINVAL);
        }

        if elf_hdr.e_ident[elf::EI_VERSION] != elf::EV_CURRENT {
            return Err(Errno::EINVAL);
        }

        // Check ABI, we don't care about ABIVERSION.
        if elf_hdr.e_ident[elf::EI_OSABI] != elf::ELFOSABI_SYSV {
            return Err(Errno::EINVAL);
        }

        // Check executable type. We don't support relocatable files.
        if elf_hdr.e_type != elf::ET_EXEC as u16 {
            return Err(Errno::EINVAL);
        }

        // Check machine type. Only load executables for this machine.
        if elf_hdr.e_machine != elf::EM_CURRENT {
            return Err(Errno::EINVAL);
        }

        // We can be certain that this is an ELF for us.
        // Now is a good time to allocate a process.
        let mut result = Process::new(is_user);
        let mut main_thread = Thread::new();

        // Start by evaluating the program headers.
        let phdrs: &[ElfPhdr] = match bytemuck::try_cast_slice(
            &data[elf_hdr.e_phoff
                ..(elf_hdr.e_phoff + elf_hdr.e_phnum as usize * size_of::<ElfPhdr>())],
        ) {
            Ok(x) => x,
            Err(x) => return Err(Errno::EINVAL),
        };

        for phdr in phdrs {
            match phdr.p_type {
                // Load the segment into memory.
                elf::PT_LOAD => {
                    // Convert the flags to our format.
                    let mut flags = VmFlags::None;
                    if phdr.p_flags & elf::PF_EXECUTE != 0 {
                        flags |= VmFlags::Exec;
                    }
                    if phdr.p_flags & elf::PF_READ != 0 {
                        flags |= VmFlags::Read;
                    }
                    if phdr.p_flags & elf::PF_WRITE != 0 {
                        flags |= VmFlags::Write;
                    }

                    let phys = match phys::alloc_bytes(phdr.p_memsz as usize) {
                        Some(x) => x,
                        None => return Err(Errno::ENOMEM),
                    };

                    result.page_table.map_range(
                        phdr.p_vaddr,
                        phys,
                        flags,
                        0,
                        phdr.p_memsz as usize,
                    )?;
                }
                // Unknown or unhandled type. Do nothing.
                _ => (),
            }
        }

        main_thread.context.set_ip(elf_hdr.e_entry);

        result.threads.push(main_thread);
        return Ok(result);
    }
}

static PID_COUNTER: AtomicUsize = AtomicUsize::new(0);
