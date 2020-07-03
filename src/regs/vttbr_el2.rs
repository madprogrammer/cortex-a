// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Virtualization Translation Table Base Register - EL2
//!
//! Holds the base address of the translation table for the initial lookup for stage 2 of an
//! address translation in the Non-secure EL1&0 translation regime, and other information
//! for this translation regime.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub VTTBR_EL2 [

        /// The VMID for the translation table.
        ///
        /// It is IMPLEMENTATION DEFINED whether the VMID is 8 bits or 16 bits.
        ///
        /// If the implementation has an 8-bit VMID, then VMID[15:8] are RES0.
        ///
        /// If the implementation has a 16-bit VMID, then:
        ///
        /// -  The VTCR_EL2.VS bit selects whether VMID[15:8] are ignored by the hardware for every
        /// purpose except reading back the register, or whether these bits are used for allocation
        /// and matching in the TLB.
        ///
        /// - The 16-bit VMID is only supported when EL2 is using AArch64. This means the hardware
        /// must ignore VMID[15:8] when EL2 is using AArch32.
        VMID  OFFSET(48) NUMBITS(16) [],

        /// Translation table base address
        BADDR OFFSET(1) NUMBITS(47) [],

        /// Common not Private
        CnP   OFFSET(0) NUMBITS(1) []
    ]
}

pub struct Reg;

impl RegisterReadWrite<u64, VTTBR_EL2::Register> for Reg {
    sys_coproc_read_raw!(u64, "VTTBR_EL2");
    sys_coproc_write_raw!(u64, "VTTBR_EL2");
}

impl Reg {
    #[inline]
    pub fn get_baddr(&self) -> u64 {
        self.read(VTTBR_EL2::BADDR) << 1
    }

    #[inline]
    pub fn set_baddr(&self, addr: u64) {
        self.write(VTTBR_EL2::BADDR.val(addr >> 1));
    }
}

pub static VTTBR_EL2: Reg = Reg {};
