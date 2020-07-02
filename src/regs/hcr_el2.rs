// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Hypervisor Configuration Register - EL2
//!
//! Provides configuration controls for virtualization, including defining
//! whether various Non-secure operations are trapped to EL2.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub HCR_EL2 [
        /// Execution state control for lower Exception levels:
        ///
        /// 0 Lower levels are all AArch32.
        /// 1 The Execution state for EL1 is AArch64. The Execution state for EL0 is determined by
        ///   the current value of PSTATE.nRW when executing at EL0.
        ///
        /// If all lower Exception levels cannot use AArch32 then this bit is RAO/WI.
        ///
        /// In an implementation that includes EL3, when SCR_EL3.NS==0, the PE behaves as if this
        /// bit has the same value as the SCR_EL3.RW bit for all purposes other than a direct read
        /// or write access of HCR_EL2.
        ///
        /// The RW bit is permitted to be cached in a TLB.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this
        /// field behaves as 1 for all purposes other than a direct read of the value of this bit.
        RW   OFFSET(31) NUMBITS(1) [
            AllLowerELsAreAarch32 = 0,
            EL1IsAarch64 = 1
        ],

        /// Default Cacheability.
        ///
        /// 0 This control has no effect on the Non-secure EL1&0 translation regime.
        ///
        /// 1 In Non-secure state:
        ///   - When EL1 is using AArch64, the PE behaves as if the value of the SCTLR_EL1.M field
        ///     is 0 for all purposes other than returning the value of a direct read of SCTLR_EL1.
        ///
        ///   - When EL1 is using AArch32, the PE behaves as if the value of the SCTLR.M field is 0
        ///     for all purposes other than returning the value of a direct read of SCTLR.
        ///
        ///   - The PE behaves as if the value of the HCR_EL2.VM field is 1 for all purposes other
        ///     than returning the value of a direct read of HCR_EL2.
        ///
        ///   - The memory type produced by stage 1 of the EL1&0 translation regime is Normal
        ///     Non-Shareable, Inner Write-Back Read-Allocate Write-Allocate, Outer Write-Back
        ///     Read-Allocate Write-Allocate.
        ///
        /// This field has no effect on the EL2, EL2&0, and EL3 translation regimes.
        ///
        /// This field is permitted to be cached in a TLB.
        ///
        /// In an implementation that includes EL3, when the value of SCR_EL3.NS is 0 the PE behaves
        /// as if this field is 0 for all purposes other than a direct read or write access of
        /// HCR_EL2.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this
        /// field behaves as 0 for all purposes other than a direct read of the value of this field.
        DC   OFFSET(12) NUMBITS(1) [],

        /// Barrier Shareability upgrade. The value in this field determines the minimum
        /// shareability domain that is applied to any barrier executed from Non-secure EL1 or EL0.
        BSU  OFFSET(10) NUMBITS(2) [
            NoEffect = 0,
            InnerShareable = 1,
            OuterShareable = 2,
            FullSystem = 3
        ],

        /// Force broadcast.  When this bit is set to 1, this causes the following instructions to
        /// be broadcast within the Inner Shareable domain when executed from Non-secure EL1:
        ///
        /// AArch32: BPIALL, TLBIALL, TLBIMVA, TLBIASID, DTLBIALL, DTLBIMVA, DTLBIASID,
        /// ITLBIALL, ITLBIMVA, ITLBIASID, TLBIMVAA, ICIALLU, TLBIMVAL, TLBIMVAAL.
        ///
        /// AArch64: TLBI VMALLE1, TLBI VAE1, TLBI ASIDE1, TLBI VAAE1, TLBI VALE1, TLBI
        /// VAALE1, IC IALLU.
        FB   OFFSET(9) NUMBITS(1) [],

        /// Virtual System Error/Asynchronous Abort
        ///
        /// 0 Virtual System Error/Asynchronous Abort is not pending by this mechanism.
        ///
        /// 1 Virtual System Error/Asynchronous Abort is pending by this mechanism.
        ///
        /// The virtual System Error/Asynchronous Abort is only enabled when the HCR_EL2.AMO bit is
        /// set.
        VSE  OFFSET(8) NUMBITS(1) [],

        /// Virtual IRQ Interrupt
        ///
        /// 0 Virtual IRQ Interrupt is not pending by this mechanism.
        ///
        /// 1 Virtual IRQ is pending by this mechanism.
        ///
        /// The virtual IRQ is only enabled when the HCR_EL2.IMO bit is set.
        VI   OFFSET(7) NUMBITS(1) [],

        /// Virtual FIQ Interrupt
        ///
        /// 0 Virtual FIQ is not pending by this mechanism.
        ///
        /// 1 Virtual FIQ is pending by this mechanism.
        ///
        /// The virtual FIQ is only enabled when the HCR_EL2.FMO bit is set.
        VF   OFFSET(6) NUMBITS(1) [],

        /// Asynchronous External Abort and SError Interrupt routing.
        ///
        /// 0 When executing at Non-secure Exception levels below EL2, physical Asynchronous
        ///   External Aborts and SError Interrupts are not taken to EL2.
        ///
        ///   When executing at EL2 using AArch64, physical Asynchronous External Aborts and
        ///   SError Interrupts are not taken unless they are routed to EL3 by the SCR_EL3.EA bit.
        ///
        ///   Virtual Asynchronous External Aborts and SError Interrupts interrupts are disabled.
        ///
        /// 1 When executing at any Exception level in Non-secure state, physical Asynchronous
        ///   External Aborts and SError Interrupts are taken to EL2 unless they are routed to EL3.
        ///
        ///   Virtual Asynchronous External Aborts and SError Interrupts are enabled in Non-secure
        ///   state.
        AMO  OFFSET(5) NUMBITS(1) [],

        /// Physical IRQ Routing
        ///
        /// 0 When executing at Non-secure Exception levels below EL2, physical IRQ interrupts are
        ///   not taken to EL2.
        ///
        ///   When executing at EL2 using AArch64, physical IRQ interrupts are not taken unless
        ///   they are routed to EL3 by the SCR_EL3.IRQ bit.
        ///
        ///   Virtual IRQ interrupts are disabled.
        ///
        /// 1 When executing at any Exception level in Non-secure state, physical IRQ interrupts
        ///   taken to EL2 unless they are routed to EL3.
        ///
        ///   Virtual IRQ interrupts are enabled in Non-secure state.
        IMO  OFFSET(4) NUMBITS(1) [],

        /// Physical FIQ Routing
        ///
        /// 0 When executing at Non-secure Exception levels below EL2, physical FIQ interrupts are
        ///   not taken to EL2.
        ///
        ///   When executing at EL2 using AArch64, physical FIQ interrupts are not taken unless
        ///   they are routed to EL3 by the SCR_EL3.FIQ bit.
        ///
        ///   Virtual FIQ interrupts are disabled.
        ///
        /// 1 When executing at any Exception level in Non-secure state, physical FIQ interrupts
        ///   taken to EL2 unless they are routed to EL3.
        ///
        ///   Virtual FIQ interrupts are enabled in Non-secure state.
        FMO  OFFSET(3) NUMBITS(1) [],

        /// Protected Table Walk.
        ///
        /// In the Non-secure EL1&0 translation regime, a translation table access made as part of a
        /// stage 1 translation table walk is subject to a stage 2 translation. The combining of the
        /// memory type attributes from the two stages of translation means the access can be made
        /// to a type of Device memory. If this occurs then the value of this bit determines the behavior:
        ///
        /// 0 The translation table walk occurs as if it is to Normal Non-cacheable memory. This
        ///   means it can be made speculatively.
        ///
        /// 1 The memory access generates a stage 2 Permission fault
        ///
        PTW  OFFSET(2) NUMBITS(1) [],

        /// Set/Way Invalidation Override. Causes Non-secure EL1 execution of the data cache
        /// invalidate by set/way instructions to perform a data cache clean and invalidate by
        /// set/way:
        ///
        /// 0 This control has no effect on the operation of data cache invalidate by set/way
        ///   instructions.
        ///
        /// 1 Data cache invalidate by set/way instructions perform a data cache clean and
        ///   invalidate by set/way.
        ///
        /// When the value of this bit is 1:
        ///
        /// AArch32: DCISW performs the same invalidation as a DCCISW instruction.
        ///
        /// AArch64: DC ISW performs the same invalidation as a DC CISW instruction.
        ///
        /// This bit can be implemented as RES 1.
        ///
        /// In an implementation that includes EL3, when the value of SCR_EL3.NS is 0 the PE behaves
        /// as if this field is 0 for all purposes other than a direct read or write access of
        /// HCR_EL2.
        ///
        /// When HCR_EL2.TGE is 1, the PE ignores the value of this field for all purposes other
        /// than a direct read of this field.
        SWIO OFFSET(1) NUMBITS(1) [],

        /// Virtualization MMU enable for Non-secure EL1 and EL0 stage 2 address translation.
        ///
        /// 0 EL1 and EL0 stage 2 address translation disabled
        ///
        /// 1 EL1 and EL0 stage 2 address translation enabbled
        ///
        VM OFFSET(0) NUMBITS(1) []
    ]
}

pub struct Reg;

impl RegisterReadWrite<u64, HCR_EL2::Register> for Reg {
    sys_coproc_read_raw!(u64, "HCR_EL2");
    sys_coproc_write_raw!(u64, "HCR_EL2");
}

pub static HCR_EL2: Reg = Reg {};
