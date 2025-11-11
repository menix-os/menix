use crate::spec::DataPointer;
use menix::static_assert;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Command {
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub namespace_id: u32,
    pub cdw2: [u32; 2],
    pub metadata: u64,
    pub data_ptr: DataPointer,
    pub payload: Payload,
}

static_assert!(size_of::<Command>() == 64);

#[derive(Clone, Copy)]
#[repr(C)]
pub union Payload {
    pub rw: ReadWriteCommand,
    pub create_cq: CreateCQCommand,
    pub create_sq: CreateSQCommand,
    pub identify: IdentifyCommand,
    pub set_features: SetFeaturesCommand,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ReadWriteCommand {
    pub start_lba: u64,
    pub length: u16,
    pub control: u16,
    pub ds_mgmt: u32,
    pub ref_tag: u32,
    pub app_tag: u16,
    pub app_mask: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CreateCQCommand {
    pub cqid: u16,
    pub queue_size: u16,
    pub cq_flags: u16,
    pub irq_vector: u16,
    __reserved2: [u32; 4],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CreateSQCommand {
    sqid: u16,
    queue_size: u16,
    sq_flags: u16,
    cqid: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IdentifyCommand {
    pub cns: u8,
    reserved: u8,
    pub controller_id: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SetFeaturesCommand {
    data: [u32; 6],
}
