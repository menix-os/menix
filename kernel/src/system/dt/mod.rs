use core::ffi::CStr;

use crate::generic::{
    boot::BootInfo,
    memory::view::{MemoryView, Register},
    util::once::Once,
};
use alloc::{slice, string::String};

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
    pub const FDT_BEGIN_NODE: u32 = 0x00000001;
    pub const FDT_END_NODE: u32 = 0x00000002;
    pub const FDT_PROP: u32 = 0x00000003;
    pub const FDT_NOP: u32 = 0x00000004;
    pub const FDT_END: u32 = 0x00000009;

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

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn root(&self) -> Node<'_> {
        Node {
            tree: self,
            name: b"/",
            start: 8,
        }
    }
}

pub struct Node<'a> {
    tree: &'a DeviceTree<'a>,
    name: &'a [u8],
    /// The offset where this node starts at, relative to the [`DeviceTree::structs`] field.
    start: usize,
}

impl<'a> Node<'a> {
    pub fn get_name(&self) -> &[u8] {
        self.name
    }

    pub fn nodes(&self) -> NodeIter<'_> {
        NodeIter {
            node: self,
            offset: self.start, // begin scanning right after node start
            depth: 0,
            done: false,
        }
    }

    pub fn properties(&self) -> PropertyIter<'_> {
        PropertyIter {
            node: self,
            offset: self.start,
        }
    }
}

/// Iterates over child nodes
pub struct NodeIter<'a> {
    node: &'a Node<'a>,
    offset: usize,
    depth: usize,
    done: bool,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let structs = self.node.tree.structs;

        while !self.done && self.offset < structs.len() {
            let tag = structs.read_reg(Register::<u32>::new(self.offset).with_be())?;
            self.offset += 4;

            match tag {
                DeviceTree::FDT_BEGIN_NODE => {
                    // parse name (NUL-terminated string)
                    let mut end = self.offset;
                    while end < structs.len() && structs[end] != 0 {
                        end += 1;
                    }
                    let name = &structs[self.offset..end];
                    self.offset = (end + 4) & !3; // align to 4
                    self.depth += 1;

                    return Some(Node {
                        tree: self.node.tree,
                        name,
                        start: self.offset,
                    });
                }
                DeviceTree::FDT_END_NODE => {
                    if self.depth == 0 {
                        self.done = true;
                        return None;
                    }
                    self.depth -= 1;
                }
                DeviceTree::FDT_PROP => {
                    let len = structs.read_reg(Register::<u32>::new(self.offset).with_be())?;
                    let nameoff =
                        structs.read_reg(Register::<u32>::new(self.offset + 4).with_be())?;
                    self.offset += 8 + ((len as usize + 3) & !3);
                }
                DeviceTree::FDT_NOP => {}
                DeviceTree::FDT_END => {
                    self.done = true;
                    return None;
                }
                _ => panic!("unknown FDT tag {:#x}", tag),
            }
        }

        None
    }
}

/// Iterates over properties of a node
pub struct PropertyIter<'a> {
    node: &'a Node<'a>,
    offset: usize,
}

impl<'a> Iterator for PropertyIter<'a> {
    type Item = Property<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let structs = self.node.tree.structs;

        while self.offset < structs.len() {
            let tag = structs.read_reg(Register::<u32>::new(self.offset).with_be())?;
            self.offset += 4;

            match tag {
                DeviceTree::FDT_PROP => {
                    let len = structs.read_reg(Register::<u32>::new(self.offset).with_be())?;
                    let nameoff =
                        structs.read_reg(Register::<u32>::new(self.offset + 4).with_be())?;
                    let data_start = self.offset + 8;
                    let data_end = data_start + len as usize;
                    let data = &structs[data_start..data_end];
                    self.offset = (data_end + 3) & !3; // align to 4

                    let name = get_str(self.node.tree.strings, nameoff)?;
                    return Some(Property {
                        tree: self.node.tree,
                        node: self.node,
                        name,
                        data,
                    });
                }
                DeviceTree::FDT_BEGIN_NODE => {
                    // skip node and descend into children.
                    return None;
                }
                DeviceTree::FDT_END_NODE => {
                    return None;
                }
                DeviceTree::FDT_NOP => {
                    continue;
                }
                DeviceTree::FDT_END => return None,
                _ => panic!("unknown FDT tag in property iter"),
            }
        }

        None
    }
}

pub struct Property<'a> {
    tree: &'a DeviceTree<'a>,
    node: &'a Node<'a>,
    name: &'a [u8],
    data: &'a [u8],
}

impl<'a> Property<'a> {
    pub fn name(&self) -> &[u8] {
        self.name
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn as_str(&self) -> Option<&[&CStr]> {
        todo!()
    }

    pub fn as_u32(&self) -> Option<&[u32]> {
        bytemuck::try_cast_slice(self.data).ok()
    }

    pub fn as_u64(&self) -> Option<&[u64]> {
        bytemuck::try_cast_slice(self.data).ok()
    }
}

fn get_str<'a>(strings: &'a [u8], off: u32) -> Option<&'a [u8]> {
    let mut end = off as usize;
    while end < strings.len() && strings[end] != 0 {
        end += 1;
    }
    Some(&strings[off as usize..end])
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

    let root = TREE.get().root();
    let model = root.properties().find(|x| x.name() == b"model").unwrap();
    log!("Running on {}", String::from_utf8_lossy(model.data()));

    log!("Found devices:");
    for node in root.nodes() {
        if let Some(dev) = node.properties().find(|x| x.name() == b"compatible") {
            log!(
                "{{ {}, compatible = \"{}\" }}",
                String::from_utf8_lossy(node.get_name()),
                String::from_utf8_lossy(dev.data)
            );
        }
    }
}
