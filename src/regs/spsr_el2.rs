// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Saved Program Status Register - EL2
//!
//! Holds the saved process state when an exception is taken to EL2.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub SPSR_EL2 [
        /// Negative condition flag.
        ///
        /// Set to the value of the N condition flag on taking an exception to EL2, and copied to
        /// the N condition flag on executing an exception return operation in EL2.
        ///
        /// Set to 1 if the result of the last flag-setting instruction was negative.
        N OFFSET(31) NUMBITS(1) [],

        /// Zero condition flag.
        ///
        /// Set to the value of the Z condition flag on taking an exception to EL2, and copied to
        /// the Z condition flag on executing an exception return operation in EL2.
        ///
        /// Set to 1 if the result of the last flag-setting instruction was zero, and to 0
        /// otherwise. A result of zero often indicates an equal result from a comparison.
        Z OFFSET(30) NUMBITS(1) [],

        /// Carry condition flag.
        ///
        /// Set to the value of the C condition flag on taking an exception to EL2, and copied to
        /// the C condition flag on executing an exception return operation in EL2.
        ///
        /// Set to 1 if the last flag-setting instruction resulted in a carry condition, for example
        /// an unsigned overflow on an addition.
        C OFFSET(29) NUMBITS(1) [],

        /// Overflow condition flag.
        ///
        /// Set to the value of the V condition flag on taking an exception to EL2, and copied to
        /// the V condition flag on executing an exception return operation in EL2.
        ///
        /// Set to 1 if the last flag-setting instruction resulted in an overflow condition, for
        /// example a signed overflow on an addition.
        V OFFSET(28) NUMBITS(1) [],

        /// Software step. Shows the value of PSTATE.SS immediately before the exception was taken.
        SS OFFSET(21) NUMBITS(1) [],

        /// Illegal Execution state bit. Shows the value of PSTATE.IL immediately before the
        /// exception was taken.
        IL OFFSET(20) NUMBITS(1) [],

        /// Process state D mask. The possible values of this bit are:
        ///
        /// 0 Watchpoint, Breakpoint, and Software Step exceptions targeted at the current Exception
        ///   level are not masked.
        ///
        /// 1 Watchpoint, Breakpoint, and Software Step exceptions targeted at the current Exception
        ///   level are masked.
        ///
        /// When the target Exception level of the debug exception is higher than the current
        /// Exception level, the exception is not masked by this bit.
        D OFFSET(9) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// SError interrupt mask bit. The possible values of this bit are:
        ///
        /// 0 Exception not masked.
        /// 1 Exception masked.
        A OFFSET(8) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// IRQ mask bit. The possible values of this bit are:
        ///
        /// 0 Exception not masked.
        /// 1 Exception masked.
        I OFFSET(7) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// FIQ mask bit. The possible values of this bit are:
        ///
        /// 0 Exception not masked.
        /// 1 Exception masked.
        F OFFSET(6) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// Execution state that the exception was taken from. The possible values of this bit are:
        ///
        /// 1 Exception taken from AArch32.
        /// 0 Exception taken from AArch64.
        M4 OFFSET(4) NUMBITS(1) [
            AArch32 = 1,
            AArch64 = 0
        ],

        /// AArch32 mode that an exception was taken from. The possible values are:
        ///
        /// M[3:0] | Mode
        /// ------------------
        /// 0b0000 | User
        /// 0b0001 | FIQ
        /// 0b0010 | IRQ
        /// 0b0011 | Supervisor
        /// 0b0111 | Abort
        /// 0b1010 | Hyp
        /// 0b1011 | Undefined
        /// 0b1111 | System
        ///
        /// Other values are reserved. The effect of programming this field to a Reserved value is
        /// that behavior is CONSTRAINED UNPREDICTABLE s described in Reserved values in System and
        /// memory-mapped registers and translation table entries on page K1-6427.
        ///
        /// AArch64 state (Exception level and selected SP) that an exception was taken from. The
        /// possible values are:
        ///
        /// M[3:0] | State
        /// --------------
        /// 0b0000 | EL0t
        /// 0b0100 | EL1t
        /// 0b0101 | EL1h
        /// 0b1000 | EL2t
        /// 0b1001 | EL2h
        ///
        /// Other values are reserved, and returning to an Exception level that is using AArch64
        /// with a reserved value in this field is treated as an illegal exception return.
        ///
        /// The bits in this field are interpreted as follows:
        ///   - M[3:2] holds the Exception Level.
        ///   - M[1] is unused and is RES 0 for all non-reserved values.
        ///   - M[0] is used to select the SP:
        ///     - 0 means the SP is always SP0.
        ///     - 1 means the exception SP is determined by the EL.
        M OFFSET(0) NUMBITS(4) [
            EL0t = 0b0000,
            EL1t = 0b0100,
            EL1h = 0b0101,
            EL2t = 0b1000,
            EL2h = 0b1001,
            FIQ  = 0b0001,
            IRQ  = 0b0010,
            Supervisor  = 0b0011,
            Abort = 0b0111,
            Hyp = 0b1010,
            Undefined = 0b1011,
            System = 0b1111

        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, SPSR_EL2::Register> for Reg {
    sys_coproc_read_raw!(u32, "SPSR_EL2");
    sys_coproc_write_raw!(u32, "SPSR_EL2");
}

pub static SPSR_EL2: Reg = Reg {};
