use crate::memory::PhysAddr;

pub trait MsiPin {
    fn get_msg_addr() -> PhysAddr;

    fn get_msg_data() -> u32;
}
