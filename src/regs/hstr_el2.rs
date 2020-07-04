// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Hypervisor System Trap Register - EL2
//!
//! Controls trapping to EL2 of Non-secure EL1 or lower AArch32 accesses to the System register in
//! the coproc == 1111 encoding space, by the CRn value used to access the register using MCR or
//! MRC instruction.
//!
//! When the register is accessible using an MCRR or MRRC instruction, this is the CRm
//! value used to access the register.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub HSTR_EL2 [
        T15 OFFSET(15) NUMBITS(1) [],
        T14 OFFSET(14) NUMBITS(1) [],
        T13 OFFSET(13) NUMBITS(1) [],
        T12 OFFSET(12) NUMBITS(1) [],
        T11 OFFSET(11) NUMBITS(1) [],
        T10 OFFSET(10) NUMBITS(1) [],
        T9  OFFSET(9) NUMBITS(1) [],
        T8  OFFSET(8) NUMBITS(1) [],
        T7  OFFSET(7) NUMBITS(1) [],
        T6  OFFSET(6) NUMBITS(1) [],
        T5  OFFSET(5) NUMBITS(1) [],
        T4  OFFSET(4) NUMBITS(1) [],
        T3  OFFSET(3) NUMBITS(1) [],
        T2  OFFSET(2) NUMBITS(1) [],
        T1  OFFSET(1) NUMBITS(1) [],
        T0  OFFSET(0) NUMBITS(1) []
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, HSTR_EL2::Register> for Reg {
    sys_coproc_read_raw!(u32, "HSTR_EL2");
    sys_coproc_write_raw!(u32, "HSTR_EL2");
}

pub static HSTR_EL2: Reg = Reg {};
