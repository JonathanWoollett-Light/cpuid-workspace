#![warn(clippy::pedantic)]
#![allow(
    non_upper_case_globals,
    non_snake_case,
    clippy::similar_names,
    clippy::unsafe_derive_deserialize
)]
//! Example
//! ```ignore
//! use test_cpuid::Cpuid;
//! let cpuid = Cpuid::new();
//! let highest_calling_parameter = cpuid
//!     .leaf0x00_highest_function_parameter_an_manufacturer_id
//!     .highest_calling_parameter;
//! let _register0_0_eax = cpuid.leaf::<0>().sub_leaf::<0>().eax();
//! let _register0_0_ebx = cpuid.leaf::<0>().sub_leaf::<0>().ebx();
//! let _register0_0_ecx = cpuid.leaf::<0>().sub_leaf::<0>().ecx();
//! let _register0_0_edx = cpuid.leaf::<0>().sub_leaf::<0>().edx();
//! ```
//! Bit flags are serialized in a little endian format e.g.
//! ```ignore
//! biflags! {
//!     pub struct MyBitFlags: u32 {
//!         const one = 1 << 0;
//!         const two = 1 << 1;
//!         const three = 1 << 2;
//!         const ten = 1 << 9;
//!         const thirty = 1 << 29;
//!     }
//! }
//! let my_bit_flags = MyBitFlags { bits: 0b0010_0000_0000_0000_0000_0010_0000_0111 };
//! assert_eq!("00100000000000000000001000000111",serde_json::to_string(&my_bit_flags).unwrap());
//! ```

use core::arch::x86_64::{CpuidResult, __cpuid, __cpuid_count};
use std::collections::HashMap;
use std::mem::transmute;
use std::{fmt, str};
mod bitflags_util;

mod cpuid_ffi;
use bitflags::bitflags;
#[allow(clippy::wildcard_imports)]
use bitflags_util::*;
pub use cpuid_ffi::*;
use log_derive::{logfn, logfn_inputs};
use serde::{Deserialize, Serialize};
// -----------------------------------------------------------------------------
// Bit flag definitions
// -----------------------------------------------------------------------------
// Leaf0x1SubLeaf0Ecx refers to the ecx value in leaf 1, sub-leaf 0 of cpuid.
#[rustfmt::skip]
bitflags! {
    // Feature Information
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x1_SubLeaf0_Ecx: u32 {
        const sse3 =        1 << 0;
        const pclmulqdq =   1 << 1;
        const dtes64 =      1 << 2;
        const monitor =     1 << 3;
        const ds_cpl =      1 << 4;
        const vmx =         1 << 5;
        const smx =         1 << 6;
        const est =         1 << 7;
        const tm2 =         1 << 8;
        const ssse3 =       1 << 9;
        const cnxt_id =     1 << 10;
        const sdbg =        1 << 11;
        const fma =         1 << 12;
        const cx16 =        1 << 13;
        const xtpr =        1 << 14;
        const pdcm =        1 << 15;
        // 16th bit reserved
        const pcid =        1 << 17;
        const dca =         1 << 18;
        const sse4_1 =      1 << 19;
        const sse4_2 =      1 << 20;
        const x2apic =      1 << 21;
        const movbe =       1 << 22;
        const popcnt =      1 << 23;
        const tsc_deadline =1 << 24;
        const aes =         1 << 25;
        const xsave =       1 << 26;
        const osxsave =     1 << 27;
        const avx =         1 << 28;
        const f16c =        1 << 29;
        const rdrnd =       1 << 30;
        const hypervisor =  1 << 31;
    }
    // Feature Information
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x1_SubLeaf0_Edx: u32 {
        const fpu =         1 << 0;
        const vme =         1 << 1;
        const de =          1 << 2;
        const pse =         1 << 3;
        const tsc =         1 << 4;
        const msr =         1 << 5;
        const pae =         1 << 6;
        const mce =         1 << 7;
        const cx8 =         1 << 8;
        const apic =        1 << 9;
        // 10th bit reserved
        const sep =         1 << 11;
        const mtrr =        1 << 12;
        const pge =         1 << 13;
        const mca =         1 << 14;
        const cmov =        1 << 15;
        const pat =         1 << 16;
        const pse_36 =      1 << 17;
        const psn =         1 << 18;
        const clfsh =       1 << 19;
        // 20th bit reserved
        const ds =          1 << 21;
        const acpi =        1 << 22;
        const mmx =         1 << 23;
        const fxsr =        1 << 24;
        const sse =         1 << 25;
        const sse2 =        1 << 26;
        const ss =          1 << 27;
        const htt =         1 << 28;
        const tm =          1 << 29;
        const ia64 =        1 << 30;
        const pbe =         1 << 31;
    }
    // Thermal and power management
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x6_SubLeaf0_Eax: u32 {
        const digital_thermal_sensor_capability =           1 << 0;
        const intel_turbo_boost_technology_capability =     1 << 1;
        const always_running_apic_timer_capability =        1 << 2;
        // 4th bit reserved
        const power_limit_notification_capability =         1 << 4;
        const extended_clock_modulation_duty_capability =   1 << 5;
        const package_thermal_management_capability =       1 << 6;
        // 7th to 31st bits reserved
    }
    // Thermal and power management
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x6_SubLeaf0_Ecx: u32 {
        const hardware_coordination_feedback_capability =   1 << 0;
        const acnt2_capability =                            1 << 1;
        // 2nd bit reserved
        const performance_energy_bias_capability =          1 << 3;
        // 4th to 31st bits reserved
    }
    // Extended Features
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x7_SubLeaf0_Ebx: u32 {
        const fsgsbase =                        1 << 0;
        // No short name
        const IA32_TSC_ADJUST =                 1 << 1;
        const sgx =                             1 << 2;
        const bmi1 =                            1 << 3;
        const hle =                             1 << 4;
        const avx2 =                            1 << 5;
        // No short name
        const FDP_EXCPTN_ONLY =                 1 << 6;
        const smep =                            1 << 7;
        const bmi2 =                            1 << 8;
        const erms =                            1 << 9;
        const invpcid =                         1 << 10;
        const rtm =                             1 << 11;
        const pqdm =                            1 << 12;
        // No short name
        const FPU_CS_and_FPU_DS_deprecated =    1 << 13;
        const mpx =                             1 << 14;
        const pqe =                             1 << 15;
        const avx512_f =                        1 << 16;
        const avx512_dq =                       1 << 17;
        const rdseed =                          1 << 18;
        const adx =                             1 << 19;
        const smap =                            1 << 20;
        const avx512_ifma =                     1 << 21;
        const pccommit =                        1 << 22;
        const clflushopt =                      1 << 23;
        const clwb =                            1 << 24;
        const intel_pt =                        1 << 25;
        const avx512_pf =                       1 << 26;
        const avx512_er =                       1 << 27;
        const avx512_cd =                       1 << 28;
        const sha =                             1 << 29;
        const avx512_bw =                       1 << 30;
        const avx512_vl =                       1 << 31;
    }
    // Extended Features
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x7_SubLeaf0_Ecx: u32 {
        const prefetchwt1 =         1 << 0;
        const avx512_vbmi =         1 << 1;
        const umip =                1 << 2;
        const pku =                 1 << 3;
        const ospke =               1 << 4;
        const waitpkg =             1 << 5;
        const avx512_vbmi2 =        1 << 6;
        const cet_ss =              1 << 7;
        const gfni =                1 << 8;
        const vaes =                1 << 9;
        const vpclmulqdq =          1 << 10;
        const avx512_vnni =         1 << 11;
        const avx512_bitalg =       1 << 12;
        const TIME_END =            1 << 13;
        const avx512_vpopcntdq =    1 << 14;
        // 15th bit reserved
        const _5_level_paging =     1 << 16;
        // mawau uses 17th to 21th bits (inclusive)
        const rdpid =               1 << 22;
        const KL =                  1 << 23;
        // 24th bit reserved
        const cldemote =            1 << 25;
        // 26th bit reserved
        const MOVDIRI =             1 << 27;
        const MOVDIR64B =           1 << 28;
        const ENQCMD =              1 << 29;
        const sgx_lc =              1 << 30;
        const pks =                 1 << 31;
    }
    // Extended Features
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x7_SubLeaf0_Edx: u32 {
        // 1st bit reserved
        // 2nd bit reserved
        const avx512_4vnniw = 1 << 2;
        const avx512_4fmaps = 1 << 3;
        const fsrm = 1 << 4;
        const uintr = 1 << 5;
        // 6th bit reserved
        // 7th bit reserved
        const avx512_vp2intersect = 1 << 8;
        const SRBDS_CTRL = 1 << 9;
        const md_clear = 1 << 10;
        const RMT_ALWAYS_ABORT = 1 << 11;
        // 12th bit reserved
        const TSX_FORCE_ABORT = 1 << 13;
        const SERIALIZE = 1 << 14;
        const Hybrid = 1 << 15;
        const TSXLDTRK = 1 << 16;
        // 17th bit reserved
        const pcconfig = 1 << 18;
        const lbr = 1 << 19;
        const cet_ibt = 1 << 20;
        // 21st bit reserved
        const amx_bf16 = 1 << 22;
        const AVX512_FP16 = 1 << 23;
        const amx_tile = 1 << 24;
        const amx_int8 = 1 << 25;
        const IBRS_IBPB_spec_ctrl = 1 << 26;
        const stibp = 1 << 27;
        const L1D_FLUSH = 1 << 28;
        const IA32_ARCH_CAPABILITIES = 1 << 29;
        const IA32_CORE_CAPABILITIES = 1 << 30;
        const ssbd = 1 << 31;
    }
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x7_SubLeaf1_Eax: u32 {
        // 0 to 3rd bits reserved
        const avx_vnni =                    1 << 4;
        const avx512_bf16 =                 1 << 5;
        // 6th to 9th bits reserved
        const fast_zero_rep_movsb =         1 << 10;
        const fast_short_rep_stosb =        1 << 11;
        const fast_short_rep_cmpsb_scasb =  1 << 12;
        // 13th to 16th bits reserved
        const fred =                        1 << 17;
        const lkgs =                        1 << 18;
        // 19th to 21th bits reserved
        const hreset =                      1 << 22;
        // 23rd to 31th bits reserved
    }
    /// <https://en.wikipedia.org/wiki/CPUID#EAX=0Dh,_ECX=1>
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0xD_SubLeaf1_Eax: u32 {
        const xsaveopt =    1 << 0;
        const xsavec =      1 << 1;
        const xgetbv_ecx1 = 1 << 2;
        const xss =         1 << 3;
        // 4th to 31st bits reserved.
    }
    /// <https://en.wikipedia.org/wiki/CPUID#EAX=12h,_ECX=0:_SGX_Leaf_Functions>
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x12_SubLeaf0_Eax: u32 {
        const sgx1 = 1 << 0;
        const sgx2 = 1 << 1;
        // 2nd to 4th bits reserved.
        const oss = 1 << 5;
        const encls = 1 << 6;
        // 7th to 31st bits reserved.
    }
    /// <https://en.wikipedia.org/wiki/CPUID#EAX=14h,_ECX=0>
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x14_SubLeaf0_Ebx: u32 {
        // 0 to 3rd bits reserved.
        const ptwrite = 1 << 4;
        // 5th to 31st bits reserved.
    }
    /// <https://en.wikipedia.org/wiki/CPUID#EAX=19h>
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x19_SubLeaf0_Ebx: u32 {
        const aes_kle = 1 << 0;
        // 1st bit reserved.
        const aes_wide_kl = 1 << 2;
        // 3rd bit reserved.
        const kl_msrs = 1 << 4;
        // 5th to 31st bits reserved.
    }
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x8000_0001_SubLeaf0_Edx: u32 {
        // Duplicates are from leaf 1 sub-leaf 0 edx.
        // 0 to 9nth bits are duplicates.
        // 10th bit is reserved.
        const syscall =     1 << 11;
        // 12th to 17th bits are duplicates.
        // 18th bit is reserved.
        const mp =          1 << 19;
        const nx =          1 << 20;
        // 21st bit is reserved.
        const mmxext =      1 << 22;
        // 23rd and 24th bits are duplicates.
        const fxsr_opt =    1 << 25;
        const pdpe1gb =     1 << 26;
        const rdtscp =      1 << 27;
        // 28th bit is reserved
        const lm =          1 << 29;
        const _3dnowext =   1 << 30;
        const _3dnow =      1 << 31;
    }
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x8000_0001_SubLeaf0_Ecx: u32 {
        const lahf =            1 << 0;
        const cmp_legacy =      1 << 1;
        const svm =             1 << 2;
        const extapic =         1 << 3;
        const cr8_legacy =      1 << 4;
        const abm =             1 << 5;
        const sse4a =           1 << 6;
        const missalignsse =    1 << 7;
        const _3dnowprefetch =  1 << 8;
        const osvw =            1 << 9;
        const ibs =             1 << 10;
        const xop =             1 << 11;
        const skinit =          1 << 12;
        const wdt =             1 << 13;
        // 14th bit reserved
        const lwp =             1 << 15;
        const fma4 =            1 << 16;
        const tce =             1 << 17;
        // 18th bit reserved
        const nodeid_msr =      1 << 19;
        const tbm =             1 << 21;
        const topoext =         1 << 22;
        const perfctr_core =    1 << 23;
        const perfctr_nb =      1 << 24;
        // 25th bit reserved
        const dbx =             1 << 26;
        const perftsc =         1 << 27;
        const pcx_l2i =         1 << 28;
        const monitorx =        1 << 29;
        const addr_mask_ext =   1 << 30;
        // 31st bit reserved
    }
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x8000_0008_SubLeaf0_Ebx: u32 {
        const clzero = 1 << 0;
        const retired_instr = 1 << 1;
        const xrstor_fp_err = 1 << 2;
        const invlpgb = 1 << 3;
        const rdpru = 1 << 4;
        // 5th to 7th bits reserved.
        const mcommit = 1 << 8;
        const wbnoinvd = 1 << 9;
        // 10th and 11th bits reserved.
        const ibpb = 1 << 12;
        const wbinvd_int = 1 << 13;
        const ibrs = 1 << 14;
        const single_thread_ibp = 1 << 15;
        // 16th bit reserved.
        const single_thread_ibp_ao = 1 << 17;
        // 18th and 19th bits reserved.
        const no_efer_lmsle = 1 << 20;
        const invlpgb_nested = 1 << 21;
        // 22nd bit reserved.
        const ppin = 1 << 23;
        const ssbd = 1 << 24;
        const virt_ssbd = 1 << 25;
        const ssb_no = 1 << 26;
        //26th to 31st bits reserved.
    }
    /// <https://en.wikipedia.org/wiki/CPUID#EAX=8000001Fh>
    #[derive(Serialize, Deserialize)]
    #[repr(C)]
    pub struct Leaf0x8000_001F_SubLeaf0_Eax: u32 {
        const sme = 1 << 0;
        const sev = 1 << 1;
        const page_flush = 1 << 2;
        const sev_es = 1 << 3;
        const sev_snp = 1 << 4;
        const vmpl = 1 << 5;
        // 6th to 9th bits are reserved.
        const hw_cache_coherency = 1 << 10;
        const _64_host = 1 << 11;
        const restricted_injection = 1 << 12;
        const alternative_injection = 1 << 13;
        const debug_swap = 1 << 14;
        const prevent_host_ibs = 1 << 15;
        const vte = 1 << 16;
        // 17th to 31st bits reserved.
    }
}

// -----------------------------------------------------------------------------
// Bitflags sub leaf impls
// -----------------------------------------------------------------------------

impl Leaf0xD_SubLeaf1_Eax {
    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Leaf0x12_SubLeaf0_Eax {
    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Leaf0x14_SubLeaf0_Ebx {
    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Leaf0x19_SubLeaf0_Ebx {
    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Leaf0x8000_001F_SubLeaf0_Eax {
    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}

// -----------------------------------------------------------------------------
// Bit flag registers impls
// -----------------------------------------------------------------------------

impl Leaf0x7_SubLeaf1_Eax {
    #[must_use]
    pub fn eax(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0xD_SubLeaf1_Eax {
    #[must_use]
    pub fn eax(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0x12_SubLeaf0_Eax {
    #[must_use]
    pub fn eax(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0x14_SubLeaf0_Ebx {
    #[must_use]
    pub fn ebx(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0x19_SubLeaf0_Ebx {
    #[must_use]
    pub fn ebx(&self) -> u32 {
        self.bits()
    }
}
impl Leaf0x8000_001F_SubLeaf0_Eax {
    #[must_use]
    pub fn eax(&self) -> u32 {
        self.bits()
    }
}

// -----------------------------------------------------------------------------
// Cpuid definition
// -----------------------------------------------------------------------------

/// <https://en.wikipedia.org/wiki/CPUID>
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Cpuid {
    /// leaf 0
    pub leaf0x00_highest_function_parameter_an_manufacturer_id:
        HighestFunctionParameterAndManufacturerID,
    /// leaf 1
    pub leaf0x01_process_info_and_feature_bits: ProcessorInfoAndFeatureBits,
    /// leaf 6
    pub leaf0x06_thermal_and_power_management: ThermalAndPowerManagement,
    /// leaf 7
    pub leaf0x07_extended_features: ExtendedFeatures,
    /// leaf 13 / 0x0D
    #[serde(with = "i")]
    pub leaf0x0d_cpuid_feature_bits: Leaf0xD_SubLeaf1_Eax,
    /// leaf 18 / 0x12h
    #[serde(with = "j")]
    pub leaf0x12_cpuid_feature_bits: Leaf0x12_SubLeaf0_Eax,
    /// leaf 20 / 0x14h
    #[serde(with = "k")]
    pub leaf0x14_cpuid_feature_bits: Leaf0x14_SubLeaf0_Ebx,
    /// leaf 25 / 0x19h
    #[serde(with = "l")]
    pub leaf0x19_cpuid_feature_bits: Leaf0x19_SubLeaf0_Ebx,
    /// leaf 0x8000_0001
    pub leaf0x8000_0001_highest_function_parameter_an_manufacturer_id:
        ExtendedProcessorInfoAndFeatureBits,
    /// leaf 0x8000_0008
    pub leaf0x8000_0008_virtual_and_physical_address_sizes: VirtualAndPhysicalAddressSizes,
    /// leaf 0x8000_001F
    #[serde(with = "p")]
    pub leaf0x8000_001F_cpuid_feature_bits: Leaf0x8000_001F_SubLeaf0_Eax,
    /// To allow usability beyond what is immediately considered at the moment, for leafs we do not 
    /// explicitly describe we store them here in the format `<(leaf,subleaf),(eax,ebx,ecx,edx)>`.
    pub misc: HashMap<(u32, u32), (u32, u32, u32, u32)>,
}
impl Cpuid {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // If the feature set of `self` covers the feature set of `other`.
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn covers(&self, other: &Self) -> bool {
        // We first check they have the same manufacturer
        self.leaf0x00_highest_function_parameter_an_manufacturer_id
            .covers(&other.leaf0x00_highest_function_parameter_an_manufacturer_id)
            && self
                .leaf0x01_process_info_and_feature_bits
                .covers(&other.leaf0x01_process_info_and_feature_bits)
            && self
                .leaf0x06_thermal_and_power_management
                .covers(&other.leaf0x06_thermal_and_power_management)
            && self
                .leaf0x07_extended_features
                .covers(&other.leaf0x07_extended_features)
            && self
                .leaf0x0d_cpuid_feature_bits
                .contains(other.leaf0x0d_cpuid_feature_bits)
            && self
                .leaf0x12_cpuid_feature_bits
                .contains(other.leaf0x12_cpuid_feature_bits)
            && self
                .leaf0x14_cpuid_feature_bits
                .contains(other.leaf0x14_cpuid_feature_bits)
            && self
                .leaf0x19_cpuid_feature_bits
                .contains(other.leaf0x19_cpuid_feature_bits)
            && self
                .leaf0x8000_0001_highest_function_parameter_an_manufacturer_id
                .covers(&other.leaf0x8000_0001_highest_function_parameter_an_manufacturer_id)
            && self
                .leaf0x8000_0008_virtual_and_physical_address_sizes
                .covers(&other.leaf0x8000_0008_virtual_and_physical_address_sizes)
            && self
                .leaf0x8000_001F_cpuid_feature_bits
                .contains(other.leaf0x8000_001F_cpuid_feature_bits)
    }

    #[must_use]
    pub fn leaf<const N: usize>(&self) -> &<Cpuid as Leaf<N>>::Output
    where
        Cpuid: Leaf<N>,
    {
        <Cpuid as Leaf<N>>::leaf(self)
    }
}

impl Default for Cpuid {
    fn default() -> Self {
        Self {
            leaf0x00_highest_function_parameter_an_manufacturer_id:
                HighestFunctionParameterAndManufacturerID::new(),
            leaf0x01_process_info_and_feature_bits: ProcessorInfoAndFeatureBits::new(),
            leaf0x06_thermal_and_power_management: ThermalAndPowerManagement::new(),
            leaf0x07_extended_features: ExtendedFeatures::new(),
            leaf0x0d_cpuid_feature_bits: {
                let CpuidResult { eax, .. } = unsafe { __cpuid_count(13, 1) };
                Leaf0xD_SubLeaf1_Eax { bits: eax }
            },
            leaf0x12_cpuid_feature_bits: {
                let CpuidResult {
                    eax,
                    ebx: _,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(18, 0) };
                Leaf0x12_SubLeaf0_Eax { bits: eax }
            },
            leaf0x14_cpuid_feature_bits: {
                let CpuidResult {
                    eax: _,
                    ebx,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(20, 0) };
                Leaf0x14_SubLeaf0_Ebx { bits: ebx }
            },
            leaf0x19_cpuid_feature_bits: {
                let CpuidResult {
                    eax: _,
                    ebx,
                    ecx: _,
                    edx: _,
                } = unsafe { __cpuid_count(25, 0) };
                Leaf0x19_SubLeaf0_Ebx { bits: ebx }
            },
            leaf0x8000_0001_highest_function_parameter_an_manufacturer_id: {
                let CpuidResult { ecx, edx, .. } = unsafe { __cpuid_count(0x8000_0001, 0) };
                ExtendedProcessorInfoAndFeatureBits {
                    edx: Leaf0x8000_0001_SubLeaf0_Edx { bits: edx },
                    ecx: Leaf0x8000_0001_SubLeaf0_Ecx { bits: ecx },
                }
            },
            leaf0x8000_0008_virtual_and_physical_address_sizes: {
                let CpuidResult { eax, ebx, ecx, .. } = unsafe { __cpuid_count(0x8000_0008, 0) };
                VirtualAndPhysicalAddressSizes {
                    eax: Leaf0x8000_0008_SubLeaf0_Eax(eax),
                    ebx: Leaf0x8000_0008_SubLeaf0_Ebx { bits: ebx },
                    ecx: Leaf0x8000_0008_SubLeaf0_Ecx(ecx),
                }
            },
            leaf0x8000_001F_cpuid_feature_bits: {
                let CpuidResult { eax, .. } = unsafe { __cpuid_count(0x8000_001F, 0) };
                Leaf0x8000_001F_SubLeaf0_Eax { bits: eax }
            },
            misc: HashMap::new(),
        }
    }
}
impl fmt::Debug for Cpuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpuid")
            .field(
                "leaf0x00_highest_function_parameter_an_manufacturer_id",
                &&self.leaf0x00_highest_function_parameter_an_manufacturer_id,
            )
            .field(
                "leaf0x01_process_info_and_feature_bits",
                &self.leaf0x01_process_info_and_feature_bits,
            )
            .field(
                "leaf0x06_thermal_and_power_management",
                &self.leaf0x06_thermal_and_power_management,
            )
            .field(
                "leaf0x07_extended_features",
                &self.leaf0x07_extended_features,
            )
            .field(
                "leaf0x0d_cpuid_feature_bits",
                &self.leaf0x0d_cpuid_feature_bits,
            )
            .field(
                "leaf0x12_cpuid_feature_bits",
                &self.leaf0x12_cpuid_feature_bits,
            )
            .field(
                "leaf0x14_cpuid_feature_bits",
                &self.leaf0x14_cpuid_feature_bits,
            )
            .field(
                "leaf0x19_cpuid_feature_bits",
                &self.leaf0x19_cpuid_feature_bits,
            )
            .field(
                "leaf0x8000_0001_highest_function_parameter_an_manufacturer_id",
                &self.leaf0x8000_0001_highest_function_parameter_an_manufacturer_id,
            )
            .field(
                "leaf0x8000_0008_virtual_and_physical_address_sizes",
                &self.leaf0x8000_0008_virtual_and_physical_address_sizes,
            )
            .field(
                "leaf0x8000_001F_cpuid_feature_bits",
                &self.leaf0x8000_001F_cpuid_feature_bits,
            )
            .finish()
    }
}
/// This conversion is lossy, this means we cannot implement `From<Cpuid> for RawCpuid`
impl TryFrom<RawCpuid> for Cpuid {
    // TODO Change this to at least `'static str` and use descriptions.
    type Error = ();

    fn try_from(cpuid: RawCpuid) -> Result<Self, Self::Error> {
        Ok(Self {
            leaf0x00_highest_function_parameter_an_manufacturer_id:
                HighestFunctionParameterAndManufacturerID::from(cpuid.get(0, 0).ok_or(())?.clone()),
            leaf0x01_process_info_and_feature_bits: ProcessorInfoAndFeatureBits::from(
                cpuid.get(1, 0).ok_or(())?.clone(),
            ),
            leaf0x06_thermal_and_power_management: ThermalAndPowerManagement::from(
                cpuid.get(6, 0).ok_or(())?.clone(),
            ),
            leaf0x07_extended_features: ExtendedFeatures::from((
                cpuid.get(7, 0).ok_or(())?.clone(),
                cpuid.get(7, 1).ok_or(())?.clone(),
            )),
            leaf0x0d_cpuid_feature_bits: {
                let RawCpuidEntry { eax, .. } = cpuid.get(13, 1).ok_or(())?;
                Leaf0xD_SubLeaf1_Eax { bits: *eax }
            },
            leaf0x12_cpuid_feature_bits: {
                let RawCpuidEntry { eax, .. } = cpuid.get(18, 0).ok_or(())?;
                Leaf0x12_SubLeaf0_Eax { bits: *eax }
            },
            leaf0x14_cpuid_feature_bits: {
                let RawCpuidEntry { ebx, .. } = cpuid.get(20, 0).ok_or(())?;
                Leaf0x14_SubLeaf0_Ebx { bits: *ebx }
            },
            leaf0x19_cpuid_feature_bits: {
                let RawCpuidEntry { ebx, .. } = cpuid.get(25, 0).ok_or(())?;
                Leaf0x19_SubLeaf0_Ebx { bits: *ebx }
            },
            leaf0x8000_0001_highest_function_parameter_an_manufacturer_id: {
                let RawCpuidEntry { ecx, edx, .. } = cpuid.get(0x8000_0001, 0).ok_or(())?;
                ExtendedProcessorInfoAndFeatureBits {
                    edx: Leaf0x8000_0001_SubLeaf0_Edx { bits: *edx },
                    ecx: Leaf0x8000_0001_SubLeaf0_Ecx { bits: *ecx },
                }
            },
            leaf0x8000_0008_virtual_and_physical_address_sizes: {
                let RawCpuidEntry { eax, ebx, ecx, .. } = cpuid.get(0x8000_0008, 0).ok_or(())?;
                VirtualAndPhysicalAddressSizes {
                    eax: Leaf0x8000_0008_SubLeaf0_Eax(*eax),
                    ebx: Leaf0x8000_0008_SubLeaf0_Ebx { bits: *ebx },
                    ecx: Leaf0x8000_0008_SubLeaf0_Ecx(*ecx),
                }
            },
            leaf0x8000_001F_cpuid_feature_bits: {
                let RawCpuidEntry { eax, .. } = cpuid.get(0x8000_001F, 0).ok_or(())?;
                Leaf0x8000_001F_SubLeaf0_Eax { bits: *eax }
            },
            misc: {
                // Filter out entries we already store explicitly
                #[allow(clippy::unnested_or_patterns)]
                cpuid
                    .iter()
                    .filter(|entry| {
                        !matches!(
                            (entry.function, entry.index),
                            (0, 0)
                                | (1, 0)
                                | (6, 0)
                                | (7, 0)
                                | (13, 1)
                                | (18, 0)
                                | (20, 0)
                                | (25, 0)
                                | (0x8000_0001, 0)
                                | (0x8000_0008, 0)
                                | (0x8000_001F, 0)
                        )
                    })
                    .map(|entry| {
                        (
                            (entry.function, entry.index),
                            (entry.eax, entry.ebx, entry.ecx, entry.edx),
                        )
                    })
                    .collect()
            },
        })
    }
}

pub trait Leaf<const INDEX: usize> {
    type Output;
    fn leaf(&self) -> &Self::Output;
}
impl Leaf<0> for Cpuid {
    type Output = HighestFunctionParameterAndManufacturerID;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x00_highest_function_parameter_an_manufacturer_id
    }
}
impl Leaf<1> for Cpuid {
    type Output = ProcessorInfoAndFeatureBits;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x01_process_info_and_feature_bits
    }
}
impl Leaf<6> for Cpuid {
    type Output = ThermalAndPowerManagement;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x06_thermal_and_power_management
    }
}
impl Leaf<7> for Cpuid {
    type Output = ExtendedFeatures;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x07_extended_features
    }
}
impl Leaf<13> for Cpuid {
    type Output = Leaf0xD_SubLeaf1_Eax;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x0d_cpuid_feature_bits
    }
}
impl Leaf<18> for Cpuid {
    type Output = Leaf0x12_SubLeaf0_Eax;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x12_cpuid_feature_bits
    }
}
impl Leaf<20> for Cpuid {
    type Output = Leaf0x14_SubLeaf0_Ebx;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x14_cpuid_feature_bits
    }
}
impl Leaf<25> for Cpuid {
    type Output = Leaf0x19_SubLeaf0_Ebx;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x19_cpuid_feature_bits
    }
}
impl Leaf<0x8000_0001> for Cpuid {
    type Output = ExtendedProcessorInfoAndFeatureBits;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x8000_0001_highest_function_parameter_an_manufacturer_id
    }
}
impl Leaf<0x8000_0008> for Cpuid {
    type Output = VirtualAndPhysicalAddressSizes;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x8000_0008_virtual_and_physical_address_sizes
    }
}
impl Leaf<0x8000_001F> for Cpuid {
    type Output = Leaf0x8000_001F_SubLeaf0_Eax;

    fn leaf(&self) -> &Self::Output {
        &self.leaf0x8000_001F_cpuid_feature_bits
    }
}

pub trait SubLeaf<const INDEX: usize> {
    type Output;
    fn sub_leaf(&self) -> &Self::Output;
}
impl SubLeaf<0> for HighestFunctionParameterAndManufacturerID {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for ProcessorInfoAndFeatureBits {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for ThermalAndPowerManagement {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for ExtendedFeatures {
    type Output = ExtendedFeaturesSubLeaf0;

    fn sub_leaf(&self) -> &Self::Output {
        &self.sub_leaf0
    }
}
impl SubLeaf<1> for ExtendedFeatures {
    type Output = Leaf0x7_SubLeaf1_Eax;

    fn sub_leaf(&self) -> &Self::Output {
        &self.sub_leaf1
    }
}
impl SubLeaf<1> for Leaf0xD_SubLeaf1_Eax {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for Leaf0x12_SubLeaf0_Eax {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for Leaf0x14_SubLeaf0_Ebx {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for Leaf0x19_SubLeaf0_Ebx {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for ExtendedProcessorInfoAndFeatureBits {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for VirtualAndPhysicalAddressSizes {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}
impl SubLeaf<0> for Leaf0x8000_001F_SubLeaf0_Eax {
    type Output = Self;

    fn sub_leaf(&self) -> &Self::Output {
        self
    }
}

/// A string wrapper around a byte array.
#[derive(Clone, Eq, PartialEq)]
#[repr(C)]
pub struct FixedString<const N: usize>(pub [u8; N]);
impl<const N: usize> fmt::Debug for FixedString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}
impl<const N: usize> Serialize for FixedString<N> {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(
            str::from_utf8(&self.0)
                .map_err(|_| serde::ser::Error::custom("invalid utf8 manufacturer id"))?,
            ser,
        )
    }
}
impl<'a, const N: usize> Deserialize<'a> for FixedString<N> {
    fn deserialize<D: serde::Deserializer<'a>>(des: D) -> Result<Self, D::Error> {
        let base = <&str>::deserialize(des)?;
        let bytes = base
            .as_bytes()
            .try_into()
            .map_err(|_| serde::de::Error::custom("incorrectly sized manufacturer id"))?;
        Ok(FixedString(bytes))
    }
}

/// <https://en.wikipedia.org/wiki/CPUID#EAX=0:_Highest_Function_Parameter_and_Manufacturer_ID>
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct HighestFunctionParameterAndManufacturerID {
    /// We use [`FixedString`] here over `[u8;12]` so it serializes to and from a string making the
    /// file more easily editable.
    pub manufacturer_id: FixedString<12>,
    pub highest_calling_parameter: u32,
}
impl HighestFunctionParameterAndManufacturerID {
    /// # Panics
    ///
    /// When `[ebx_bytes, edx_bytes, ecx_bytes].concat().try_into::<[u;12]>()` returns `Err` which
    /// never occurs since `ebx_bytes`, `ebx_bytes` and `ecx_bytes` are all `[u8;4]`s.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// # Panics
    ///
    /// When `self.manufacturer_id.0[8..12].try_into::<[u8;4]>()` is `Err`, which never occurs since
    /// `self.manufacturer_id.0` is `[u8;12]`.
    #[must_use]
    pub fn ebx(&self) -> u32 {
        u32::from_ne_bytes(unsafe { self.manufacturer_id.0[0..4].try_into().unwrap_unchecked() })
    }

    /// # Panics
    ///
    /// When `self.manufacturer_id.0[8..12].try_into::<[u8;4]>()` is `Err`, which never occurs since
    /// `self.manufacturer_id.0` is `[u8;12]`.
    #[must_use]
    pub fn edx(&self) -> u32 {
        u32::from_ne_bytes(unsafe { self.manufacturer_id.0[4..8].try_into().unwrap_unchecked() })
    }

    /// # Panics
    ///
    /// When `self.manufacturer_id.0[8..12].try_into::<[u8;4]>()` is `Err`, which never occurs since
    /// `self.manufacturer_id.0` is `[u8;12]`.
    #[must_use]
    pub fn ecx(&self) -> u32 {
        u32::from_ne_bytes(unsafe { self.manufacturer_id.0[8..12].try_into().unwrap_unchecked() })
    }

    #[must_use]
    pub fn eax(&self) -> u32 {
        self.highest_calling_parameter
    }

    /// Since we do not currently need to support cross paltform snapshots (AMD <-> Intel) we can
    /// simply require the mnanufactuer id's match.
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.manufacturer_id == other.manufacturer_id
            && self.highest_calling_parameter >= other.highest_calling_parameter
    }

    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl fmt::Debug for HighestFunctionParameterAndManufacturerID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HighestFunctionParameterAndManufacturerID")
            .field("manufacturer_id", &self.manufacturer_id)
            .field("highest_calling_parameter", &self.highest_calling_parameter)
            .finish()
    }
}
impl Default for HighestFunctionParameterAndManufacturerID {
    fn default() -> Self {
        let CpuidResult { eax, ebx, ecx, edx } = unsafe { __cpuid(0) };
        Self::from((eax, ebx, ecx, edx))
    }
}
impl From<RawCpuidEntry> for HighestFunctionParameterAndManufacturerID {
    fn from(entry: RawCpuidEntry) -> Self {
        let RawCpuidEntry {
            eax, ebx, ecx, edx, ..
        } = entry;
        Self::from((eax, ebx, ecx, edx))
    }
}
/// From `(eax,ebx,ecx,edx)`.
impl From<(u32, u32, u32, u32)> for HighestFunctionParameterAndManufacturerID {
    fn from((eax, ebx, ecx, edx): (u32, u32, u32, u32)) -> Self {
        let (ebx_bytes, edx_bytes, ecx_bytes) =
            (ebx.to_ne_bytes(), edx.to_ne_bytes(), ecx.to_ne_bytes());
        let vec = [ebx_bytes, edx_bytes, ecx_bytes].concat();
        Self {
            manufacturer_id: FixedString(unsafe { vec.try_into().unwrap_unchecked() }),
            highest_calling_parameter: eax,
        }
    }
}

/// <https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits>
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct ProcessorInfoAndFeatureBits {
    #[serde(with = "processor_version_information_mod")]
    pub processor_version_information: ProcessorVersionInformation,
    pub additional_information: AdditionalInformation,
    pub feature_information: FeatureInformation,
}
impl ProcessorInfoAndFeatureBits {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn ebx(&self) -> u32 {
        u32::from_ne_bytes([
            self.additional_information.brand_index,
            self.additional_information.clflush_line_size,
            self.additional_information
                .maximum_addressable_logical_processor_ids,
            self.additional_information.local_apic_id,
        ])
    }

    #[must_use]
    pub fn edx(&self) -> u32 {
        self.feature_information.edx.bits
    }

    #[must_use]
    pub fn ecx(&self) -> u32 {
        self.feature_information.ecx.bits
    }

    #[must_use]
    pub fn eax(&self) -> u32 {
        self.processor_version_information.0
    }

    // If the feature set of `self` covers the feature set of `other`.
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.processor_version_information
            .covers(&other.processor_version_information)
            && self
                .additional_information
                .covers(&other.additional_information)
            && self.feature_information.covers(&other.feature_information)
    }

    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Default for ProcessorInfoAndFeatureBits {
    fn default() -> Self {
        let CpuidResult { eax, ebx, ecx, edx } = unsafe { __cpuid_count(1, 0) };
        Self::from((eax, ebx, ecx, edx))
    }
}
impl From<RawCpuidEntry> for ProcessorInfoAndFeatureBits {
    fn from(entry: RawCpuidEntry) -> Self {
        let RawCpuidEntry {
            eax, ebx, ecx, edx, ..
        } = entry;
        Self::from((eax, ebx, ecx, edx))
    }
}
/// From `(eax,ebx,ecx,edx)`.
impl From<(u32, u32, u32, u32)> for ProcessorInfoAndFeatureBits {
    fn from((eax, ebx, ecx, edx): (u32, u32, u32, u32)) -> Self {
        Self {
            processor_version_information: ProcessorVersionInformation(eax),
            additional_information: unsafe { transmute::<_, AdditionalInformation>(ebx) },
            feature_information: FeatureInformation {
                ecx: Leaf0x1_SubLeaf0_Ecx { bits: ecx },
                edx: Leaf0x1_SubLeaf0_Edx { bits: edx },
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct ProcessorVersionInformation(u32);
impl ProcessorVersionInformation {
    #[must_use]
    pub fn stepping_id(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_0000_0000_0000_0000_1111) as u8
    }

    #[must_use]
    pub fn model(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_0000_0000_1111_0000) >> 4) as u8
    }

    #[must_use]
    pub fn family_id(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8) as u8
    }

    #[must_use]
    pub fn processor_type(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_0011_0000_0000_0000) >> 12) as u8
    }

    #[must_use]
    pub fn extended_model_id(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_1111_0000_0000_0000_0000) >> 16) as u8
    }

    #[must_use]
    pub fn extended_family_id(&self) -> u8 {
        ((self.0 & 0b0000_1111_1111_0000_0000_0000_0000_0000) >> 20) as u8
    }

    /// # Errors
    ///
    /// Errors when the given value `x` is greater than or equal to `16`
    /// (`if x < 16 { ... Ok(()) } else { Err(...) }`).
    pub fn set_stepping_id(&mut self, x: u8) -> Result<(), &str> {
        if x < 16 {
            self.0 = (self.0 & !0b0000_0000_0000_0000_0000_0000_0000_1111) | u32::from(x);
            Ok(())
        } else {
            Err("x >= 16")
        }
    }

    /// # Errors
    ///
    /// Errors when the given value `x` is greater than or equal to `16`
    /// (`if x < 16 { ... Ok(()) } else { Err(...) }`).
    pub fn set_model(&mut self, x: u8) -> Result<(), &str> {
        if x < 16 {
            self.0 = (self.0 & !0b0000_0000_0000_0000_0000_0000_1111_0000) | (u32::from(x) << 4);
            Ok(())
        } else {
            Err("x >= 16")
        }
    }

    /// # Errors
    ///
    /// Errors when the given value `x` is greater than or equal to `16`
    /// (`if x < 16 { ... Ok(()) } else { Err(...) }`).
    pub fn set_family_id(&mut self, x: u8) -> Result<(), &str> {
        if x < 16 {
            self.0 = (self.0 & !0b0000_0000_0000_0000_0000_1111_0000_0000) | (u32::from(x) << 8);
            Ok(())
        } else {
            Err("x >= 16")
        }
    }

    /// # Errors
    ///
    /// Errors when the given value `x` is greater than or equal to `4`
    /// (`if x < 4 { ... Ok(()) } else { Err(...) }`).
    pub fn set_processor_type(&mut self, x: u8) -> Result<(), &str> {
        if x < 4 {
            self.0 = (self.0 & !0b0000_0000_0000_0000_0011_0000_0000_0000) | (u32::from(x) << 12);
            Ok(())
        } else {
            Err("x >= 4")
        }
    }

    /// # Errors
    ///
    /// Errors when the given value `x` is greater than or equal to `16`
    /// (`if x < 16 { ... Ok(()) } else { Err(...) }`).
    pub fn set_extended_model_id(&mut self, x: u8) -> Result<(), &str> {
        if x < 16 {
            self.0 = (self.0 & !0b0000_0000_0000_1111_0000_0000_0000_0000) | (u32::from(x) << 16);
            Ok(())
        } else {
            Err("x >= 16")
        }
    }

    pub fn set_extended_family_id(&mut self, x: u8) {
        self.0 = (self.0 & !0b0000_1111_1111_0000_0000_0000_0000_0000) | (u32::from(x) << 20);
    }

    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl TryFrom<HashMap<&str, u8>> for ProcessorVersionInformation {
    type Error = String;

    fn try_from(value: HashMap<&str, u8>) -> Result<Self, Self::Error> {
        let mut base = Self(0);
        base.set_stepping_id(*value.get("stepping_id").ok_or("`stepping_id` not found")?)?;
        base.set_model(*value.get("model").ok_or("`model` not found")?)?;
        base.set_family_id(*value.get("family_id").ok_or("`family_id` not found")?)?;
        base.set_processor_type(
            *value
                .get("processor_type")
                .ok_or("`processor_type` not found")?,
        )?;
        base.set_extended_model_id(
            *value
                .get("extended_model_id")
                .ok_or("`extended_model_id` not found")?,
        )?;
        base.set_extended_family_id(
            *value
                .get("extended_family_id")
                .ok_or("`extended_family_id` not found")?,
        );
        Ok(base)
    }
}
impl fmt::Debug for ProcessorVersionInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcessorVersionInformation")
            .field("stepping_id", &self.stepping_id())
            .field("model", &self.model())
            .field("family_id", &self.family_id())
            .field("processor_type", &self.processor_type())
            .field("extended_model_id", &self.extended_model_id())
            .field("extended_family_id", &self.extended_family_id())
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct AdditionalInformation {
    pub brand_index: u8,
    pub clflush_line_size: u8,
    /// Local APIC ID
    pub maximum_addressable_logical_processor_ids: u8,
    pub local_apic_id: u8,
}
impl AdditionalInformation {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.brand_index == other.brand_index
            && self.clflush_line_size == other.clflush_line_size
            && self.maximum_addressable_logical_processor_ids
                >= other.maximum_addressable_logical_processor_ids
        // This value doesn't directly relate to available functionlity
        // && self.local_apic_id == other.local_apic_id
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct FeatureInformation {
    #[serde(with = "a")]
    pub ecx: Leaf0x1_SubLeaf0_Ecx,
    #[serde(with = "b")]
    pub edx: Leaf0x1_SubLeaf0_Edx,
}
impl FeatureInformation {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.ecx.contains(other.ecx) && self.edx.contains(other.edx)
    }
}
impl fmt::Debug for FeatureInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.ecx.is_empty(), self.edx.is_empty()) {
            (true, true) => write!(f, "(empty)"),
            (false, true) => write!(f, "{:?}", self.ecx),
            (true, false) => write!(f, "{:?}", self.edx),
            (false, false) => write!(f, "{:?} | {:?}", self.ecx, self.edx),
        }
    }
}
/// <https://en.wikipedia.org/wiki/CPUID#EAX=6:_Thermal_and_power_management>
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct ThermalAndPowerManagement {
    pub features: ThermalAndPowerManagementFeatures,
    pub number_of_interrupt_thresholds: Leaf6SubLeaf0Ebx,
}
impl ThermalAndPowerManagement {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn ebx(&self) -> u32 {
        self.number_of_interrupt_thresholds.0
    }

    #[must_use]
    pub fn ecx(&self) -> u32 {
        self.features.ecx.bits()
    }

    #[must_use]
    pub fn eax(&self) -> u32 {
        self.features.eax.bits()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.features.covers(&other.features)
            && self
                .number_of_interrupt_thresholds
                .covers(&other.number_of_interrupt_thresholds)
    }

    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Default for ThermalAndPowerManagement {
    fn default() -> Self {
        let CpuidResult { eax, ebx, ecx, edx } = unsafe { __cpuid_count(6, 0) };
        Self::from((eax, ebx, ecx, edx))
    }
}
impl From<RawCpuidEntry> for ThermalAndPowerManagement {
    fn from(entry: RawCpuidEntry) -> Self {
        let RawCpuidEntry {
            eax, ebx, ecx, edx, ..
        } = entry;
        Self::from((eax, ebx, ecx, edx))
    }
}
/// From `(eax,ebx,ecx,edx)`.
impl From<(u32, u32, u32, u32)> for ThermalAndPowerManagement {
    fn from((eax, ebx, ecx, _edx): (u32, u32, u32, u32)) -> Self {
        Self {
            features: ThermalAndPowerManagementFeatures {
                eax: Leaf0x6_SubLeaf0_Eax { bits: eax },
                ecx: Leaf0x6_SubLeaf0_Ecx { bits: ecx },
            },
            number_of_interrupt_thresholds: Leaf6SubLeaf0Ebx(ebx),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct ThermalAndPowerManagementFeatures {
    #[serde(with = "c")]
    pub eax: Leaf0x6_SubLeaf0_Eax,
    #[serde(with = "d")]
    pub ecx: Leaf0x6_SubLeaf0_Ecx,
}
impl ThermalAndPowerManagementFeatures {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.eax.contains(other.eax) && self.ecx.contains(other.ecx)
    }
}
impl fmt::Debug for ThermalAndPowerManagementFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.eax.is_empty(), self.ecx.is_empty()) {
            (true, true) => write!(f, "(empty)"),
            (false, true) => write!(f, "{:?}", self.eax),
            (true, false) => write!(f, "{:?}", self.ecx),
            (false, false) => write!(f, "{:?} | {:?}", self.eax, self.ecx),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Leaf6SubLeaf0Ebx(u32);
impl Leaf6SubLeaf0Ebx {
    fn number_of_interrupt_thresholds(&self) -> u8 {
        (self.0 & 0b0000_1111) as u8
    }

    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.number_of_interrupt_thresholds() >= other.number_of_interrupt_thresholds()
    }
}
impl fmt::Debug for Leaf6SubLeaf0Ebx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number_of_interrupt_thresholds())
    }
}

/// <https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features> & <https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=1:_Extended_Features>
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct ExtendedFeatures {
    pub sub_leaf0: ExtendedFeaturesSubLeaf0,
    #[serde(with = "h")]
    pub sub_leaf1: Leaf0x7_SubLeaf1_Eax,
}
impl ExtendedFeatures {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.sub_leaf0.covers(&other.sub_leaf0) && self.sub_leaf1.contains(other.sub_leaf1)
    }

    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl Default for ExtendedFeatures {
    fn default() -> Self {
        let CpuidResult {
            eax: eax0,
            ebx: ebx0,
            ecx: ecx0,
            edx: edx0,
        } = unsafe { __cpuid_count(7, 0) };
        let CpuidResult {
            eax: eax1,
            ebx: ebx1,
            ecx: ecx1,
            edx: edx1,
        } = unsafe { __cpuid_count(7, 1) };
        Self::from(((eax0, ebx0, ecx0, edx0), (eax1, ebx1, ecx1, edx1)))
    }
}
impl From<(RawCpuidEntry, RawCpuidEntry)> for ExtendedFeatures {
    fn from((one, two): (RawCpuidEntry, RawCpuidEntry)) -> Self {
        let RawCpuidEntry {
            eax: eax0,
            ebx: ebx0,
            ecx: ecx0,
            edx: edx0,
            ..
        } = one;
        let RawCpuidEntry {
            eax: eax1,
            ebx: ebx1,
            ecx: ecx1,
            edx: edx1,
            ..
        } = two;
        Self::from(((eax0, ebx0, ecx0, edx0), (eax1, ebx1, ecx1, edx1)))
    }
}
/// From `(eax,ebx,ecx,edx)`.
impl From<((u32, u32, u32, u32), (u32, u32, u32, u32))> for ExtendedFeatures {
    fn from(
        ((_eax0, ebx0, ecx0, edx0), (eax1, _ebx1, _ecx1, _edx1)): (
            (u32, u32, u32, u32),
            (u32, u32, u32, u32),
        ),
    ) -> Self {
        Self {
            sub_leaf0: ExtendedFeaturesSubLeaf0 {
                ebx: Leaf0x7_SubLeaf0_Ebx { bits: ebx0 },
                ecx: Leaf0x7_SubLeaf0_Ecx { bits: ecx0 },
                edx: Leaf0x7_SubLeaf0_Edx { bits: edx0 },
            },
            sub_leaf1: Leaf0x7_SubLeaf1_Eax { bits: eax1 },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct ExtendedFeaturesSubLeaf0 {
    #[serde(with = "e")]
    pub ebx: Leaf0x7_SubLeaf0_Ebx,
    #[serde(with = "f")]
    pub ecx: Leaf0x7_SubLeaf0_Ecx,
    #[serde(with = "g")]
    pub edx: Leaf0x7_SubLeaf0_Edx,
}
impl ExtendedFeaturesSubLeaf0 {
    #[must_use]
    pub fn ebx(&self) -> u32 {
        self.ebx.bits()
    }

    #[must_use]
    pub fn ecx(&self) -> u32 {
        self.ecx.bits()
    }

    #[must_use]
    pub fn edx(&self) -> u32 {
        self.edx.bits()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.ebx.contains(other.ebx) && self.ecx.contains(other.ecx) && self.edx.contains(other.edx)
    }
}
impl fmt::Debug for ExtendedFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ebx_printed = if self.sub_leaf0.ebx.is_empty() {
            false
        } else {
            write!(f, "{:?}", self.sub_leaf0.ebx)?;
            true
        };
        let ecx_printed = if self.sub_leaf0.ecx.is_empty() {
            false
        } else {
            if ebx_printed {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.sub_leaf0.ecx)?;
            true
        };
        let edx_printed = if self.sub_leaf0.edx.is_empty() {
            false
        } else {
            if ecx_printed {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.sub_leaf0.edx)?;
            true
        };
        let _eax_printed = if self.sub_leaf1.is_empty() {
            false
        } else {
            if edx_printed {
                write!(f, " | ")?;
            }
            write!(f, "{:?}", self.sub_leaf1)?;
            true
        };
        Ok(())
    }
}

/// <https://en.wikipedia.org/wiki/CPUID#EAX=80000001h:_Extended_Processor_Info_and_Feature_Bits>
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct ExtendedProcessorInfoAndFeatureBits {
    #[serde(with = "m")]
    pub edx: Leaf0x8000_0001_SubLeaf0_Edx,
    #[serde(with = "n")]
    pub ecx: Leaf0x8000_0001_SubLeaf0_Ecx,
}
impl ExtendedProcessorInfoAndFeatureBits {
    #[must_use]
    pub fn edx(&self) -> u32 {
        self.edx.bits()
    }

    #[must_use]
    pub fn ecx(&self) -> u32 {
        self.ecx.bits()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.edx.contains(other.edx) && self.ecx.contains(other.ecx)
    }

    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl fmt::Debug for ExtendedProcessorInfoAndFeatureBits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.edx.is_empty(), self.ecx.is_empty()) {
            (true, true) => write!(f, "(empty)"),
            (false, true) => write!(f, "{:?}", self.edx),
            (true, false) => write!(f, "{:?}", self.ecx),
            (false, false) => write!(f, "{:?} | {:?}", self.edx, self.ecx),
        }
    }
}

/// <https://en.wikipedia.org/wiki/CPUID#EAX=80000008h:_Virtual_and_Physical_address_Sizes>
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct VirtualAndPhysicalAddressSizes {
    #[serde(with = "leaf0x8000_0008_sub_leaf0_eax_mod")]
    pub eax: Leaf0x8000_0008_SubLeaf0_Eax,
    #[serde(with = "o")]
    pub ebx: Leaf0x8000_0008_SubLeaf0_Ebx,
    #[serde(with = "leaf0x8000_0008_sub_leaf0_ecx_mod")]
    pub ecx: Leaf0x8000_0008_SubLeaf0_Ecx,
}
impl VirtualAndPhysicalAddressSizes {
    #[must_use]
    pub fn eax(&self) -> u32 {
        self.eax.0
    }

    #[must_use]
    pub fn ebx(&self) -> u32 {
        self.ebx.bits()
    }

    #[must_use]
    pub fn ecx(&self) -> u32 {
        self.ecx.0
    }

    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.eax.covers(&other.eax) && self.ebx.contains(other.ebx) && self.ecx.covers(&other.ecx)
    }

    #[must_use]
    pub fn sub_leaf<const N: usize>(&self) -> &<Self as SubLeaf<N>>::Output
    where
        Self: SubLeaf<N>,
    {
        <Self as SubLeaf<N>>::sub_leaf(self)
    }
}
impl fmt::Debug for VirtualAndPhysicalAddressSizes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VirtualAndPhysicalAddressSizes")
            .field(
                "number_of_physical_address_bits",
                &self.eax.number_of_physical_address_bits(),
            )
            .field(
                "number_of_linear_address_bits",
                &self.eax.number_of_linear_address_bits(),
            )
            .field("features", &self.ebx)
            .field(
                "number_of_physical_cores_minus_1",
                &self.ecx.number_of_physical_cores_minus_1(),
            )
            .field(
                "log2_of_maximum_apic_id",
                &self.ecx.log2_of_maximum_apic_id(),
            )
            .field(
                "performance_timestamp_counter_size",
                &self.ecx.performance_timestamp_counter_size(),
            )
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Leaf0x8000_0008_SubLeaf0_Eax(u32);
impl Leaf0x8000_0008_SubLeaf0_Eax {
    #[must_use]
    pub fn number_of_physical_address_bits(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u8
    }

    #[must_use]
    pub fn number_of_linear_address_bits(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_1111_1111_0000_0000) >> 8) as u8
    }

    pub fn set_number_of_physical_address_bits(&mut self, x: u8) {
        self.0 = (self.0 & !0b0000_0000_0000_0000_0000_0000_1111_1111) | u32::from(x);
    }

    pub fn set_number_of_linear_address_bits(&mut self, x: u8) {
        self.0 = (self.0 & !0b0000_0000_0000_0000_1111_1111_0000_0000) | (u32::from(x) << 8);
    }

    // 16th to 31st bits reserved
    /// Covers:
    ///
    /// > The only problem that would appear would be if the CPU on which the snapshot was created
    /// > has a larger address space than the one where the snapshot is being restored. In this case
    /// > we would need to abort the restore since the new CPU can’t reach memory that is mapped
    /// > outside its address space.
    ///
    /// and
    ///
    /// > CPUs support different address sizes. Typically with microVMs we don’t allocate physical
    /// > memory to cover the whole address space of the CPU. Regardless, we shouldn’t resume a
    /// > microVM on a host with smaller address size if it was snapshotted on a host with a larger
    /// > address size.
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.number_of_physical_address_bits() >= other.number_of_physical_address_bits()
            && self.number_of_linear_address_bits() >= other.number_of_linear_address_bits()
    }
}
impl TryFrom<HashMap<&str, u8>> for Leaf0x8000_0008_SubLeaf0_Eax {
    type Error = String;

    fn try_from(value: HashMap<&str, u8>) -> Result<Self, Self::Error> {
        let mut base = Self(0);
        base.set_number_of_physical_address_bits(
            *value
                .get("number_of_physical_address_bits")
                .ok_or("`number_of_physical_address_bits` not found")?,
        );
        base.set_number_of_linear_address_bits(
            *value
                .get("number_of_linear_address_bits")
                .ok_or("`number_of_linear_address_bits` not found")?,
        );
        Ok(base)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Leaf0x8000_0008_SubLeaf0_Ecx(u32);
impl Leaf0x8000_0008_SubLeaf0_Ecx {
    #[must_use]
    pub fn number_of_physical_cores_minus_1(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u8
    }

    // 8th to 11th bits reserved
    #[must_use]
    pub fn log2_of_maximum_apic_id(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0000_1111_0000_0000_0000) >> 12) as u8
    }

    #[must_use]
    pub fn performance_timestamp_counter_size(&self) -> u8 {
        ((self.0 & 0b0000_0000_0000_0011_0000_0000_0000_0000) >> 16) as u8
    }

    pub fn set_number_of_physical_cores_minus_1(&mut self, x: u8) {
        self.0 = (self.0 & !0b0000_0000_0000_0000_0000_0000_1111_1111) | u32::from(x);
    }

    // 8th to 11th bits reserved
    /// # Errors
    ///
    /// Errors when the given value `x` is greater than or equal to `16`
    /// (`if x < 16 { ... Ok(()) } else { Err(...) }`).
    pub fn set_log2_of_maximum_apic_id(&mut self, x: u8) -> Result<(), &str> {
        if x < 16 {
            self.0 = (self.0 & !0b0000_0000_0000_0000_1111_0000_0000_0000) | ((u32::from(x)) << 12);
            Ok(())
        } else {
            Err("x >= 16")
        }
    }

    /// # Errors
    ///
    /// Errors when the given value `x` is greater than or equal to `4`
    /// (`if x < 4 { ... Ok(()) } else { Err(...) }`).
    pub fn set_performance_timestamp_counter_size(&mut self, x: u8) -> Result<(), &str> {
        if x < 4 {
            self.0 = (self.0 & !0b0000_0000_0000_0011_0000_0000_0000_0000) | ((u32::from(x)) << 16);
            Ok(())
        } else {
            Err("x >= 4")
        }
    }

    // 18th to 31st bits reserved
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    fn covers(&self, other: &Self) -> bool {
        self.number_of_physical_cores_minus_1() >= other.number_of_physical_cores_minus_1()
            && self.log2_of_maximum_apic_id() >= other.log2_of_maximum_apic_id()
            && self.performance_timestamp_counter_size()
                >= other.performance_timestamp_counter_size()
    }
}
impl TryFrom<HashMap<&str, u8>> for Leaf0x8000_0008_SubLeaf0_Ecx {
    type Error = String;

    fn try_from(value: HashMap<&str, u8>) -> Result<Self, Self::Error> {
        let mut base = Self(0);
        base.set_number_of_physical_cores_minus_1(
            *value
                .get("number_of_physical_cores_minus_1")
                .ok_or("`number_of_physical_cores_minus_1` not found")?,
        );
        base.set_log2_of_maximum_apic_id(
            *value
                .get("log2_of_maximum_apic_id")
                .ok_or("`log2_of_maximum_apic_id` not found")?,
        )?;
        base.set_performance_timestamp_counter_size(
            *value
                .get("performance_timestamp_counter_size")
                .ok_or("`performance_timestamp_counter_size` not found")?,
        )?;
        Ok(base)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{read_to_string, File};
    use std::io::Write;
    use std::sync::Once;

    use simple_logger::SimpleLogger;

    static INIT_LOGGER: Once = Once::new();

    /// Setup function that is only run once, even if called multiple times.
    fn init_logger() {
        INIT_LOGGER.call_once(|| {
            SimpleLogger::new().init().unwrap();
        });
    }

    use super::*;
    #[test]
    fn print() {
        init_logger();
        let cpuid = Cpuid::new();
        println!("cpuid: {:#?}", cpuid);
    }
    #[test]
    fn serialize_deserialzie() {
        init_logger();
        let cpuid = Cpuid::new();
        println!("cpuid: {:#?}", cpuid);
        let serialized = serde_json::to_string_pretty(&cpuid).unwrap();
        let mut file = File::create("cpuid-x86_64.json").unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
        drop(file);

        let reserialized = read_to_string("cpuid-x86_64.json").unwrap();
        let deserialized: Cpuid = serde_json::from_str(&reserialized).unwrap();
        println!("deserialized: {:#?}", deserialized);
        assert_eq!(cpuid, deserialized);
    }
    // #[test]
    // fn checking() {
    //     let hold = unsafe
    // {Leaf0x8000_0001_SubLeaf0_Edx::from_bits_unchecked(0b00101111110100111111101111111111)};
    //     println!("hold: {:?}",hold);
    // }

    #[test]
    fn leaf_trait_index() {
        init_logger();
        let cpuid = Cpuid::new();
        let _leaf0 = Leaf::<0>::leaf(&cpuid);
        let _leaf1 = Leaf::<1>::leaf(&cpuid);
        let _leaf6 = Leaf::<6>::leaf(&cpuid);
        let _leaf7 = Leaf::<7>::leaf(&cpuid);
        let _leaf13 = Leaf::<13>::leaf(&cpuid);
        let _leaf18 = Leaf::<18>::leaf(&cpuid);
        let _leaf20 = Leaf::<20>::leaf(&cpuid);
        let _leaf25 = Leaf::<25>::leaf(&cpuid);
        let _leaf0x8000_0001 = Leaf::<0x8000_0001>::leaf(&cpuid);
        let _leaf0x8000_0008 = Leaf::<0x8000_0008>::leaf(&cpuid);
        let _leaf0x8000_001F = Leaf::<0x8000_001F>::leaf(&cpuid);
    }
    #[test]
    fn leaf_fn_index() {
        init_logger();
        let cpuid = Cpuid::new();
        let _leaf0 = cpuid.leaf::<0>();
        let _leaf1 = cpuid.leaf::<1>();
        let _leaf6 = cpuid.leaf::<6>();
        let _leaf7 = cpuid.leaf::<7>();
        let _leaf13 = cpuid.leaf::<13>();
        let _leaf18 = cpuid.leaf::<18>();
        let _leaf20 = cpuid.leaf::<20>();
        let _leaf25 = cpuid.leaf::<25>();
        let _leaf0x8000_0001 = cpuid.leaf::<0x8000_0001>();
        let _leaf0x8000_0008 = cpuid.leaf::<0x8000_0008>();
        let _leaf0x8000_001F = cpuid.leaf::<0x8000_001F>();
    }
    #[test]
    fn sub_leaf_fn_index() {
        init_logger();
        let cpuid = Cpuid::new();
        let _sub_leaf0_0 = cpuid.leaf::<0>().sub_leaf::<0>();
        let _sub_leaf1_0 = cpuid.leaf::<1>().sub_leaf::<0>();
        let _sub_leaf6_0 = cpuid.leaf::<6>().sub_leaf::<0>();
        let _sub_leaf7_0 = cpuid.leaf::<7>().sub_leaf::<0>();
        let _sub_leaf7_1 = cpuid.leaf::<7>().sub_leaf::<1>();
        let _sub_leaf13_1 = cpuid.leaf::<13>().sub_leaf::<1>();
        let _sub_leaf18_0 = cpuid.leaf::<18>().sub_leaf::<0>();
        let _sub_leaf20_0 = cpuid.leaf::<20>().sub_leaf::<0>();
        let _sub_leaf0x8000_0001_0 = cpuid.leaf::<0x8000_0001>().sub_leaf::<0>();
        let _sub_leaf0x8000_0008_0 = cpuid.leaf::<0x8000_0008>().sub_leaf::<0>();
        let _sub_leaf0x8000_001F_0 = cpuid.leaf::<0x8000_001F>().sub_leaf::<0>();
    }
    #[test]
    fn registers_access() {
        init_logger();
        let cpuid = Cpuid::new();

        let _register0_0_eax = cpuid.leaf::<0>().sub_leaf::<0>().eax();
        let _register0_0_ebx = cpuid.leaf::<0>().sub_leaf::<0>().ebx();
        let _register0_0_ecx = cpuid.leaf::<0>().sub_leaf::<0>().ecx();
        let _register0_0_edx = cpuid.leaf::<0>().sub_leaf::<0>().edx();

        let _register1_0_eax = cpuid.leaf::<1>().sub_leaf::<0>().eax();
        let _register1_0_ebx = cpuid.leaf::<1>().sub_leaf::<0>().ebx();
        let _register1_0_ecx = cpuid.leaf::<1>().sub_leaf::<0>().ecx();
        let _register1_0_edx = cpuid.leaf::<1>().sub_leaf::<0>().edx();

        let _register6_0_eax = cpuid.leaf::<6>().sub_leaf::<0>().eax();
        let _register6_0_ebx = cpuid.leaf::<6>().sub_leaf::<0>().ebx();
        let _register6_0_ecx = cpuid.leaf::<6>().sub_leaf::<0>().ecx();

        let _register7_0_ebx = cpuid.leaf::<7>().sub_leaf::<0>().ebx();
        let _register7_0_ecx = cpuid.leaf::<7>().sub_leaf::<0>().ecx();
        let _register7_0_edx = cpuid.leaf::<7>().sub_leaf::<0>().edx();

        let _register7_1_eax = cpuid.leaf::<7>().sub_leaf::<1>().eax();

        let _register_13_1_eax = cpuid.leaf::<13>().sub_leaf::<1>().eax();

        let _register18_0_eax = cpuid.leaf::<18>().sub_leaf::<0>().eax();

        let _register20_0_ebx = cpuid.leaf::<20>().sub_leaf::<0>().ebx();

        let _register0x8000_0001_0_ecx = cpuid.leaf::<0x8000_0001>().sub_leaf::<0>().ecx();
        let _register0x8000_0001_0_edx = cpuid.leaf::<0x8000_0001>().sub_leaf::<0>().edx();

        let _register0x8000_0008_0_eax = cpuid.leaf::<0x8000_0008>().sub_leaf::<0>().eax();
        let _register0x8000_0008_0_ebx = cpuid.leaf::<0x8000_0008>().sub_leaf::<0>().ebx();
        let _register0x8000_0008_0_ecx = cpuid.leaf::<0x8000_0008>().sub_leaf::<0>().ecx();

        let _register0x8000_001F_0_eax = cpuid.leaf::<0x8000_001F>().sub_leaf::<0>().eax();
    }
}
