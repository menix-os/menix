use crate::{
    error::NvmeError,
    queue::Queue,
    spec::{self, DataPointer},
};
use menix::memory::{BitValue, UnsafeMemoryView};

pub trait Command {
    unsafe fn write_command(&self, view: &impl UnsafeMemoryView) -> Result<(), NvmeError>;
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TheCommand {
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub namespace_id: u32,
    pub cdw2: [u32; 2],
    pub metadata: u64,
    pub data_ptr: DataPointer,
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

pub struct CreateCQCommand<'a> {
    pub queue: &'a Queue,
    pub cqid: u16,
    pub queue_size: u16,
    pub irqs_enabled: bool,
    pub irq_vector: u16,
}

impl Command for CreateCQCommand<'_> {
    unsafe fn write_command(&self, view: &impl UnsafeMemoryView) -> Result<(), NvmeError> {
        unsafe {
            let cdw0 = BitValue::new(0)
                .write_field(spec::sq_entry::OPC, spec::admin_cmd::CREATE_CQ)
                .write_field(spec::sq_entry::PSDT, 1); // We always want to use PRP for this.

            view.write_reg(spec::sq_entry::DPTR0, self.queue.get_cq_addr().into());
            let cdw10 = BitValue::new(0)
                .write_field(spec::sq_entry::create_cq::QSIZE, self.queue_size)
                .write_field(spec::sq_entry::create_cq::QID, self.cqid);
            let cdw11 = BitValue::new(0)
                .write_field(spec::sq_entry::create_cq::IV, self.irq_vector)
                .write_field(
                    spec::sq_entry::create_cq::IEN,
                    if self.irqs_enabled { 1 } else { 0 },
                ) // TODO: Enable interrupts.
                .write_field(spec::sq_entry::create_cq::PC, 1); // Our buffer is physically contiguous.

            view.write_reg(spec::sq_entry::CDW0, cdw0.value());
            view.write_reg(spec::sq_entry::CDW10, cdw10.value());
            view.write_reg(spec::sq_entry::CDW10, cdw11.value());
        }

        Ok(())
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CreateSQCommand {
    pub sqid: u16,
    pub queue_size: u16,
    pub sq_flags: u16,
    pub cqid: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IdentifyCommand {
    pub cns: u8,
    pub reserved: u8,
    pub controller_id: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SetFeaturesCommand {
    data: [u32; 6],
}
