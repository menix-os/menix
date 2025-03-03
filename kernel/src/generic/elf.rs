use super::virt::GenericPageMap;

/// Loads a raw ELF executable from memory into a given page map.
pub fn load_from_memory(map: &mut impl GenericPageMap, data: &[u8]) {}
