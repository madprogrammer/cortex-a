// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Current Cache Size ID Register - EL1
//!
//! Provides information about the architecture of the currently selected cache.

use register::{cpu::RegisterReadOnly, register_bitfields};

register_bitfields! {u32,
    pub CCSIDR_EL1 [
        /// Number of sets in cache
        ///
        /// (Number of sets in cache) - 1, therefore a value of 0 indicates 1 set in the cache. The
        /// number of sets does not have to be a power of 2.
        NumSets OFFSET(13) NUMBITS(15) [],

        /// Associativity of the cache
        ///
        /// (Associativity of cache) - 1, therefore a value of 0 indicates an associativity of 1.
        /// The associativity does not have to be a power of 2.
        Associativity OFFSET(3) NUMBITS(10) [],

        /// Line size
        /// (Log2(Number of bytes in cache line)) - 4. For example:
        ///
        /// For a line length of 16 bytes: Log2(16) = 4, LineSize entry = 0. This is the minimum
        /// line length.
        ///
        /// For a line length of 32 bytes: Log2(32) = 5, LineSize entry = 1
        LineSize OFFSET(0) NUMBITS(3) []
    ]
}

pub struct Reg;

impl RegisterReadOnly<u32, CCSIDR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "CCSIDR_EL1");
}

pub static CCSIDR_EL1: Reg = Reg {};
