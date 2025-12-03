use crate::{
    device::block::BlockDevice,
    memory::{
        AllocFlags, KernelAlloc, MmioView, PageAllocator, PhysAddr, UnsafeMemoryView, VirtAddr,
    },
    posix::errno::{EResult, Errno},
    vfs::File,
};
use alloc::{string::String, sync::Arc, vec::Vec};

pub struct GptPartition {
    /// The backing block device.
    block_device: Arc<dyn BlockDevice>,
    /// The first LBA of this partition.
    start_lba: u64,
    /// The last LBA of this partition.
    end_lba: u64,
    /// GUID of the partition type.
    type_guid: [u8; 16],
    /// Unique GUID of the partition.
    unique_guid: [u8; 16],
    /// Unique GUID of the partition.
    name: [u16; 36],
}

pub mod header {
    use crate::memory::view::Register;

    pub const REVISION: Register<u32> = Register::new(0x8).with_le();
    pub const HEADER_SIZE: Register<u32> = Register::new(0xC).with_le();
    pub const ENTRIES_LBA: Register<u64> = Register::new(0x48).with_le();
    pub const NUM_PARTS: Register<u32> = Register::new(0x50).with_le();
    pub const SIZEOF_ENTRY: Register<u32> = Register::new(0x54).with_le();
}

pub mod entry {
    use crate::memory::view::Register;

    pub const START_LBA: Register<u64> = Register::new(0x20).with_le();
    pub const END_LBA: Register<u64> = Register::new(0x28).with_le();
    pub const ATTRIBUTES: Register<u64> = Register::new(0x30).with_le();
}

impl GptPartition {
    pub fn scan(device: Arc<dyn BlockDevice>) -> EResult<Vec<Self>> {
        let mut partitions = Vec::new();
        let block = KernelAlloc::alloc_bytes(device.get_lba_size(), AllocFlags::Zeroed)?;
        let view = unsafe { MmioView::new(block, device.get_lba_size()) };

        'block: {
            // Read the GPT header.
            if device.read_lba(block, 1, 1)? != 1 {
                break 'block;
            }
            let slice = unsafe { view.as_slice() };
            if &slice[0..8] != b"EFI PART" {
                break 'block;
            }

            let revision = unsafe { view.read_reg(header::REVISION).unwrap() };
            let header_size = unsafe { view.read_reg(header::HEADER_SIZE).unwrap() };
            let entries_lba = unsafe { view.read_reg(header::ENTRIES_LBA).unwrap() };
            let num_parts = unsafe { view.read_reg(header::NUM_PARTS).unwrap() };
            let sizeof_entry = unsafe { view.read_reg(header::SIZEOF_ENTRY).unwrap() };

            // Read the partition entries.
            let entries_per_lba = device.get_lba_size() / sizeof_entry.value() as usize;
            for i in 0..num_parts.value() {
                let offset = (i as usize % entries_per_lba) * sizeof_entry.value() as usize;
                let view = view.sub_view(offset).unwrap();
                // Every `entries_per_lba` entries we need to read a new LBA.
                if i.is_multiple_of(entries_per_lba as u32)
                    && device.read_lba(
                        block,
                        1,
                        entries_lba.value() + (i as u64 / entries_per_lba as u64),
                    )? != 1
                {
                    break 'block;
                }

                let start_lba = unsafe { view.read_reg(entry::START_LBA).unwrap() };
                let end_lba = unsafe { view.read_reg(entry::END_LBA).unwrap() };
                let attributes = unsafe { view.read_reg(entry::ATTRIBUTES).unwrap() };

                let mut type_guid = [const { 0 }; 16];
                type_guid.copy_from_slice(&slice[offset..][..16]);

                let mut unique_guid = [const { 0 }; 16];
                unique_guid.copy_from_slice(&slice[offset + 16..][..16]);

                let mut name = [const { 0u16 }; 36];
                name.copy_from_slice(bytemuck::cast_slice(&slice[offset + 56..][..72]));

                // Ignore unused entries.
                if type_guid == [const { 0 }; 16] {
                    continue;
                }

                partitions.push(GptPartition {
                    block_device: device.clone(),
                    start_lba: start_lba.value(),
                    end_lba: end_lba.value(),
                    type_guid,
                    unique_guid,
                    name,
                });
            }
        }

        unsafe { KernelAlloc::dealloc_bytes(block, device.get_lba_size()) };
        Ok(partitions)
    }

    pub fn name(&self) -> String {
        String::from_utf16_lossy(&self.name)
    }
}

impl BlockDevice for GptPartition {
    fn get_lba_size(&self) -> usize {
        self.block_device.get_lba_size()
    }

    fn read_lba(&self, buffer: PhysAddr, num_lba: usize, lba: u64) -> EResult<usize> {
        if self.start_lba + lba > self.end_lba {
            return Ok(0);
        }

        self.block_device
            .read_lba(buffer, num_lba, self.start_lba + lba)
    }

    fn write_lba(&self, buffer: PhysAddr, lba: u64) -> EResult<()> {
        self.block_device.write_lba(buffer, self.start_lba + lba)
    }

    fn handle_ioctl(&self, file: &File, request: usize, arg: VirtAddr) -> EResult<usize> {
        _ = (arg, request, file);
        Err(Errno::ENODEV)
    }
}
