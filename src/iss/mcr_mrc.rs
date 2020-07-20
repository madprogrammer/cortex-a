use register::{cpu::RegisterReadOnly, register_bitfields};

// ISS encoding for an exception from an MCR or MRC access
//
// This encoding is used by:
// - Trapped MCR or MRC access with (coproc==1111) that is not reported using EC 0b000000.
// - Trapped MCR or MRC access with (coproc==1110).
// - Trapped VMRS access, from ID group trap, that is not reported using EC 0b000111
register_bitfields! {u32,
    pub ISS [
        CV OFFSET(24) NUMBITS(1) [
            NotValid = 0,
            Valid = 1
        ],

        /// The condition code for the trapped instruction. This field is valid only for exceptions
        /// taken from AArch32, and only when the value of CV is 1.
        ///
        /// For exceptions taken from AArch64, this field is set to 0b1110.
        ///
        /// For exceptions taken from AArch32:
        /// - When an A32 instruction is trapped, CV is set to 1 and:
        ///   - If the instruction is conditional, COND is set to the condition code field value from
        ///     the instruction.
        ///   - If the instruction is unconditional, COND is set to 0b1110.
        /// - A conditional A32 instruction that is known to pass its condition code check can be
        /// presented either:
        /// - With COND set to 0b1110, the value for unconditional.
        /// - With the COND value held in the instruction.
        /// - When a T32 instruction is trapped, it is IMPLEMENTATION DEFINED whether:
        ///   - CV is set to 0 and COND is set to an UNKNOWN value. Software must examine the
        ///     SPSR.IT field to determine the condition, if any, of the T32 instruction.
        ///   - CV is set to 1 and COND is set to the condition code for the condition that applied
        ///     to the instruction.
        /// - For an implementation that, for both A32 and T32 instructions, takes an exception on
        ///   a trapped conditional instruction only if the instruction passes its condition code
        ///   check, these definitions mean that when CV is set to 1 it is IMPLEMENTATION DEFINED
        ///   whether the COND field is set to 0b1110, or to the value of any condition that applied
        ///   to the instruction.
        Cond OFFSET(20) NUMBITS(4) [],

        /// The Opc2 value from the issued instruction.
        ///
        /// For a trapped VMRS access, holds the value 0b000.
        Opc2 OFFSET(17) NUMBITS(3) [],

        /// The Opc1 value from the issued instruction.
        ///
        /// For a trapped VMRS access, holds the value 0b111.
        Opc1 OFFSET(14) NUMBITS(3) [],

        /// The CRn value from the issued instruction.
        ///
        /// For a trapped VMRS access, holds the reg field from the VMRS instruction encoding.
        CRn OFFSET(10) NUMBITS(4) [],

        /// The Rt value from the issued instruction, the general-purpose register used
        /// for the transfer. The reported value gives the AArch64 view of the register.
        Rt  OFFSET(5) NUMBITS(5) [],

        /// The CRm value from the issued instruction.
        /// For a trapped VMRS access, holds the value 0b0000.
        CRm OFFSET(1) NUMBITS(4) [],

        /// Indicates the direction of the trapped instruction.
        ///
        /// The possible values of this bit are:
        /// 0 Write to System register space. MCR instruction.
        /// 1 Read from System register space. MRC or VMRS instruction.
        Direction OFFSET(0) NUMBITS(1) [
            SystemRegisterWrite = 0,
            SystemRegisterRead = 1
        ]
    ]
}

pub struct McrMrcAccessIss {
    value: u32
}

impl RegisterReadOnly<u32, ISS::Register> for McrMrcAccessIss {
	#[inline(always)]
    fn get(&self) -> u32 {
        self.value
    }
}

impl McrMrcAccessIss {
    pub fn new(value: u32) -> McrMrcAccessIss {
        McrMrcAccessIss { value: value }
    }
}
