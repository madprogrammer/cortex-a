// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Cache Size Selection Register - EL1
//!
//! Selects the current Cache Size ID Register, CCSIDR_EL1, by specifying the required cache level
//! and the cache type (either instruction or data cache).

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub CSSELR_EL1 [
        /// Cache Level
        ///
        /// If CSSELR_EL1.Level is programmed to a cache level that is not implemented, then the
        /// value for this field on a read of CSSELR is UNKNOWN.
        Level OFFSET(1) NUMBITS(3) [
            Level1 = 0b000,
            Level2 = 0b001,
            Level3 = 0b010,
            Level4 = 0b011,
            Level5 = 0b100,
            Level6 = 0b101,
            Level7 = 0b110
        ],

        /// Instruction not Data bit. Permitted values are:
        ///
        /// 0 Data or unified cache.
        /// 1 Instruction cache.
        ///
        /// If CSSELR_EL1.Level is programmed to a cache level that is not implemented, then the
        /// value for
        /// this field on a read of CSSELR is UNKNOWN.
        InD OFFSET(0) NUMBITS(1) [
            DataOrUnifiedCache = 0,
            InstructionCache = 1
        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, CSSELR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "CSSELR_EL1");
    sys_coproc_write_raw!(u32, "CSSELR_EL1");
}

pub static CSSELR_EL1: Reg = Reg {};
