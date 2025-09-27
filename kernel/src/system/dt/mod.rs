use crate::generic::memory::view::Register;
use crate::generic::{boot::BootInfo, memory::view::MemoryView, util::once::Once};
use alloc::slice;

pub struct DeviceTree<'a> {
    version: u32,
    compat_version: u32,
    boot_cpuid: u32,
    structs: &'a [u8],
    strings: &'a [u8],
}

impl<'a> DeviceTree<'a> {
    pub const MAGIC_VALUE: u32 = 0xD00DFEED;
    pub const MAGIC: Register<u32> = Register::new(0).with_be();
    pub const TOTAL_SIZE: Register<u32> = Register::new(4).with_be();
    pub const STRUCTS_OFFSET: Register<u32> = Register::new(8).with_be();
    pub const STRINGS_OFFSET: Register<u32> = Register::new(12).with_be();
    pub const RESERVED_OFFSET: Register<u32> = Register::new(16).with_be();
    pub const VERSION: Register<u32> = Register::new(20).with_be();
    pub const COMPAT_VERSION: Register<u32> = Register::new(24).with_be();
    pub const BOOT_CPUID: Register<u32> = Register::new(28).with_be();
    pub const STRINGS_SIZE: Register<u32> = Register::new(32).with_be();
    pub const STRUCTS_SIZE: Register<u32> = Register::new(36).with_be();

    pub fn try_new(data: &'a [u8]) -> Option<Self> {
        if data.read_reg(Self::MAGIC)? != Self::MAGIC_VALUE {
            return None;
        }

        Some(Self {
            version: data.read_reg(Self::VERSION)?,
            compat_version: data.read_reg(Self::COMPAT_VERSION)?,
            boot_cpuid: data.read_reg(Self::BOOT_CPUID)?,
            structs: &data[data.read_reg(Self::STRUCTS_OFFSET)? as _..]
                [..data.read_reg(Self::STRUCTS_SIZE)? as _],
            strings: &data[data.read_reg(Self::STRINGS_OFFSET)? as _..]
                [..data.read_reg(Self::STRINGS_SIZE)? as _],
        })
    }

    pub fn find_node(&self, path: &[u8]) -> Option<DeviceTreeNode<'_>> {
        todo!()
    }
}

pub struct DeviceTreeNode<'a> {
    tree: &'a DeviceTree<'a>,
}

impl<'a> DeviceTreeNode<'a> {
    pub fn find_property(&self, name: &[u8]) -> Option<DeviceTreeProperty<'_>> {
        todo!()
    }
}

pub struct DeviceTreeProperty<'a> {
    tree: &'a DeviceTree<'a>,
    node: &'a DeviceTreeNode<'a>,
}

impl<'a> DeviceTreeProperty<'a> {
    pub fn as_slice(&self) -> &[u8] {
        todo!()
    }
}

pub static TREE: Once<DeviceTree> = Once::new();

#[initgraph::task(
    name = "system.dt.parse-blob",
    depends = [crate::generic::memory::MEMORY_STAGE],
    entails = [crate::INIT_STAGE]
)]
fn TREE_STAGE() {
    let Some(fdt_addr) = BootInfo::get().fdt_addr else {
        return;
    };

    unsafe {
        let slice = slice::from_raw_parts_mut(fdt_addr.as_hhdm(), 8);
        let len = slice.read_reg(DeviceTree::TOTAL_SIZE).unwrap();
        let slice = slice::from_raw_parts_mut(fdt_addr.as_hhdm(), len as _);

        TREE.init(DeviceTree::try_new(slice).expect("Failed to parse DTB"));
    }
}
