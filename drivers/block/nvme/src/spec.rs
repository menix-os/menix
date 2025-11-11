#![allow(unused)]

use menix::static_assert;

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
    /// Data Pointer 0
    pub const DPTR0: Register<u64> = Register::new(24).with_le();
    /// Data Pointer 1
    pub const DPTR1: Register<u64> = Register::new(32).with_le();

    pub const CDW10: Register<u32> = Register::new(40).with_le();
    pub const CDW11: Register<u32> = Register::new(44).with_le();
    pub const CDW12: Register<u32> = Register::new(48).with_le();
    pub const CDW13: Register<u32> = Register::new(52).with_le();
    pub const CDW14: Register<u32> = Register::new(56).with_le();
    pub const CDW15: Register<u32> = Register::new(60).with_le();
}

pub mod cq_entry {
    use menix::memory::{Field, Register};

    pub const SIZE: usize = 0x10;

    pub const DW0: Register<u32> = Register::new(0).with_le();
    pub const DW2: Register<u32> = Register::new(8).with_le();

    pub const SQ_IDENT: Field<u32, u16> = Field::new_bits(DW2, 16..=31);
    pub const SQ_HEAD: Field<u32, u16> = Field::new_bits(DW2, 0..=15);

    pub const DW3: Register<u32> = Register::new(12).with_le();

    pub const STATUS: Field<u32, u16> = Field::new_bits(DW3, 17..=31);
    pub const PHASE_TAG: Field<u32, u8> = Field::new_bits(DW3, 16..=16);
    pub const CID: Field<u32, u16> = Field::new_bits(DW3, 0..=15);
}

pub mod regs {
    use menix::memory::Register;

    /// Controller Capabilities
    pub const CAP: Register<u64> = Register::new(0x00).with_le();

    pub mod cap {
        use menix::memory::Field;

        /// Memory Page Size Maximum
        pub const MPSMAX: Field<u64, u8> = Field::new_bits(super::CAP, 52..=55);
        /// Memory Page Size Minimum
        pub const MPSMIN: Field<u64, u8> = Field::new_bits(super::CAP, 48..=51);
        /// Boot Partition Support
        pub const BPS: Field<u64, u8> = Field::new_bits(super::CAP, 45..=45);
        /// Command Sets Supported
        pub const CSS: Field<u64, u8> = Field::new_bits(super::CAP, 37..=44);
        /// NVM Subsystem Reset Supported
        pub const NSSRS: Field<u64, u8> = Field::new_bits(super::CAP, 36..=36);
        /// Doorbell Stride
        pub const DSTRD: Field<u64, u8> = Field::new_bits(super::CAP, 32..=35);
        /// Timeout
        pub const TO: Field<u64, u8> = Field::new_bits(super::CAP, 24..=31);
        /// Arbitration Mechanism Supported
        pub const AMS: Field<u64, u8> = Field::new_bits(super::CAP, 17..=18);
        /// Contiguous Queues Required
        pub const CQR: Field<u64, u8> = Field::new_bits(super::CAP, 16..=16);
        /// Maximum Queue Entries Supported
        pub const MQES: Field<u64, u16> = Field::new_bits(super::CAP, 00..=15);
    }

    /// Version
    pub const VS: Register<u32> = Register::new(0x08).with_le();

    pub mod vs {
        use menix::memory::Field;

        /// Major Version Number
        pub const MJR: Field<u32, u16> = Field::new_bits(super::VS, 16..=31);
        /// Minor Version Number
        pub const MNR: Field<u32, u8> = Field::new_bits(super::VS, 08..=15);
        /// Tertiary Version Number
        pub const TER: Field<u32, u8> = Field::new_bits(super::VS, 00..=07);
    }

    /// Interrupt Mask Set
    pub const INTMS: Register<u32> = Register::new(0x0C).with_le();
    /// Interrupt Mask Clear
    pub const INTMC: Register<u32> = Register::new(0x10).with_le();

    /// Controller Configuration
    pub const CC: Register<u32> = Register::new(0x14).with_le();

    pub mod cc {
        use menix::memory::Field;

        /// I/O Completion Queue Entry Size
        pub const IOCQES: Field<u32, u8> = Field::new_bits(super::CC, 20..=23);
        /// I/O Submission Queue Entry Size
        pub const IOSQES: Field<u32, u8> = Field::new_bits(super::CC, 16..=19);
        /// Shutdown Notification
        pub const SHN: Field<u32, u8> = Field::new_bits(super::CC, 14..=15);
        /// Arbitration Mechanism Selected
        pub const AMS: Field<u32, u8> = Field::new_bits(super::CC, 11..=13);
        /// Memory Page Size
        pub const MPS: Field<u32, u8> = Field::new_bits(super::CC, 07..=10);
        /// I/O Command Set Selected
        pub const CSS: Field<u32, u8> = Field::new_bits(super::CC, 04..=06);
        /// Enable
        pub const EN: Field<u32, u8> = Field::new_bits(super::CC, 00..=00);
    }

    /// Controller Status
    pub const CSTS: Register<u32> = Register::new(0x1C).with_le();

    pub mod csts {
        use menix::memory::Field;

        /// Processing Paused
        pub const PP: Field<u32, u8> = Field::new_bits(super::CSTS, 05..=05);
        /// NVM Subsystem Reset Occurred
        pub const NSSRO: Field<u32, u8> = Field::new_bits(super::CSTS, 04..=04);
        /// Shutdown Status
        pub const SHST: Field<u32, u8> = Field::new_bits(super::CSTS, 02..=03);
        /// Controller Fatal Status
        pub const CFS: Field<u32, u8> = Field::new_bits(super::CSTS, 01..=01);
        /// Ready
        pub const RDY: Field<u32, u8> = Field::new_bits(super::CSTS, 00..=00);
    }

    /// Admin Queue Attributes
    pub const AQA: Register<u32> = Register::new(0x24).with_le();

    pub mod aqa {
        use menix::memory::Field;

        /// Admin Completion Queue Size
        pub const ACQS: Field<u32, u16> = Field::new_bits(super::AQA, 00..=11);
        /// Admin Submission Queue Size
        pub const ASQS: Field<u32, u16> = Field::new_bits(super::AQA, 16..=27);
    }

    pub const ASQ: Register<u64> = Register::new(0x28).with_le();

    pub const ACQ: Register<u64> = Register::new(0x30).with_le();
}

/// Generic Command Set
pub mod cmd {
    pub const FLUSH: u8 = 0x00;
    pub const WRITE: u8 = 0x01;
    pub const READ: u8 = 0x02;
}

/// Admin Command Set
pub mod admin_cmd {
    pub const DELETE_SQ: u8 = 0x00;
    pub const CREATE_SQ: u8 = 0x01;
    pub const DELETE_CQ: u8 = 0x04;
    pub const CREATE_CQ: u8 = 0x05;
    pub const IDENTIFY: u8 = 0x06;
    pub const ABORT: u8 = 0x08;
    pub const SET_FEATURES: u8 = 0x09;
    pub const GET_FEATURES: u8 = 0x0A;
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DataPointer {
    prp1: u64,
    prp2: u64,
}

/// An entry in the completion queue.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CompletionEntry {
    result: u64,
    sq_head: u16,
    sq_id: u16,
    cmd_id: u16,
    status: CompletionStatus,
}
static_assert!(size_of::<CompletionEntry>() == 16);

/// A status value returned in a [`CompletionEntry`].
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct CompletionStatus {
    status: u16,
}

impl CompletionStatus {
    pub fn is_success(&self) -> bool {
        self.status == 0
    }
}

enum CompletionCode {
    Generic = 0x00,
    CommandSpecific = 0x01,
    MediaAndDataIntegrityError = 0x02,
    PathRelated = 0x03,
    VendorSpecific = 0x07,
}
