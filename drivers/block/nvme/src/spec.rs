#![allow(unused)]

/// Generic Command Set
pub mod cmd {
    pub const NVME_CMD_FLUSH: u8 = 0x00;
    pub const NVME_CMD_WRITE: u8 = 0x01;
    pub const NVME_CMD_READ: u8 = 0x02;
}

/// Admin Command Set
pub mod admin_cmd {
    pub const NVME_ACMD_DELETE_SQ: u8 = 0x00;
    pub const NVME_ACMD_CREATE_SQ: u8 = 0x01;
    pub const NVME_ACMD_DELETE_CQ: u8 = 0x04;
    pub const NVME_ACMD_CREATE_CQ: u8 = 0x05;
    pub const NVME_ACMD_IDENTIFY: u8 = 0x06;
    pub const NVME_ACMD_ABORT: u8 = 0x08;
    pub const NVME_ACMD_SET_FEATURES: u8 = 0x09;
    pub const NVME_ACMD_GET_FEATURES: u8 = 0x0A;
}

/// Submission queue entry.
pub mod sq_entry {
    use menix::memory::{Field, Register};

    pub const CDW0: Register<u32> = Register::new(0).with_le();

    /// Command Identifier
    pub const CID: Field<u32, u16> = Field::new_bits(CDW0, 16..=31);
    /// PRP or SGL for Data Transfer
    pub const PSDT: Field<u32, u8> = Field::new_bits(CDW0, 14..=15);
    /// Fused Operation
    pub const FUSE: Field<u32, u8> = Field::new_bits(CDW0, 08..=09);
    /// Opcode
    pub const OPC: Field<u32, u8> = Field::new_bits(CDW0, 00..=07);

    /// Namespace Identifier
    pub const NSID: Register<u32> = Register::new(4).with_le();
    /// Metadata Pointer
    pub const MPTR: Register<u64> = Register::new(16).with_le();
    /// Data Pointer
    pub const DPTR: Register<u64> = Register::new(24).with_le();

    pub const CDW10: Register<u32> = Register::new(40).with_le();
    pub const CDW11: Register<u32> = Register::new(44).with_le();
    pub const CDW12: Register<u32> = Register::new(48).with_le();
    pub const CDW13: Register<u32> = Register::new(52).with_le();
    pub const CDW14: Register<u32> = Register::new(56).with_le();
    pub const CDW15: Register<u32> = Register::new(60).with_le();
}

pub mod regs {
    use menix::memory::{Field, Register};

    /// Controller Capabilities
    pub const CAP: Register<u64> = Register::new(0x00).with_le();

    /// Memory Page Size Maximum
    pub const MPSMAX: Field<u64, u8> = Field::new_bits(CAP, 52..=55);
    /// Memory Page Size Minimum
    pub const MPSMIN: Field<u64, u8> = Field::new_bits(CAP, 48..=51);
    /// Boot Partition Support
    pub const BPS: Field<u64, u8> = Field::new_bits(CAP, 45..=45);
    /// Command Sets Supported
    pub const CSS: Field<u64, u8> = Field::new_bits(CAP, 37..=44);
    /// NVM Subsystem Reset Supported
    pub const NSSRS: Field<u64, u8> = Field::new_bits(CAP, 36..=36);
    /// Doorbell Stride
    pub const DSTRD: Field<u64, u8> = Field::new_bits(CAP, 32..=35);
    /// Timeout
    pub const TO: Field<u64, u8> = Field::new_bits(CAP, 24..=31);
    /// Arbitration Mechanism Supported
    pub const AMS: Field<u64, u8> = Field::new_bits(CAP, 17..=18);
    /// Contiguous Queues Required
    pub const CQR: Field<u64, u8> = Field::new_bits(CAP, 16..=16);
    /// Maximum Queue Entries Supported
    pub const MQES: Field<u64, u16> = Field::new_bits(CAP, 00..=15);

    /// Version
    pub const VS: Register<u32> = Register::new(0x08).with_le();
    /// Major Version Number
    pub const MJR: Field<u32, u16> = Field::new_bits(VS, 16..=31);
    /// Minor Version Number
    pub const MNR: Field<u32, u8> = Field::new_bits(VS, 08..=15);
    /// Tertiary Version Number
    pub const TER: Field<u32, u8> = Field::new_bits(VS, 00..=07);

    /// Interrupt Mask Set
    pub const INTMS: Register<u32> = Register::new(0x0C).with_le();
    /// Interrupt Mask Clear
    pub const INTMC: Register<u32> = Register::new(0x10).with_le();
}
