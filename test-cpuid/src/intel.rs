use std::fmt;

use bit_fields::bitfield;
use log_derive::{logfn, logfn_inputs};

use super::{FixedString, RawCpuid};

static KEYWORDS: phf::Map<u8, &'static str> = phf::phf_map! {
    0x00u8 => "Null descriptor, this byte contains no information",
    0x01u8 => "Instruction TLB: 4 KByte pages, 4-way set associative, 32 entries",
    0x02u8 => "Instruction TLB: 4 MByte pages, fully associative, 2 entries",
    0x03u8 => "Data TLB: 4 KByte pages, 4-way set associative, 64 entries",
    0x04u8 => "Data TLB: 4 MByte pages, 4-way set associative, 8 entries",
    0x05u8 => "Data TLB1: 4 MByte pages, 4-way set associative, 32 entries",
    0x06u8 => "1st-level instruction cache: 8 KBytes, 4-way set associative, 32 byte line size",
    0x08u8 => "1st-level instruction cache: 16 KBytes, 4-way set associative, 32 byte line size",
    0x09u8 => "1st-level instruction cache: 32KBytes, 4-way set associative, 64 byte line size",
    0x0Au8 => "1st-level data cache: 8 KBytes, 2-way set associative, 32 byte line size",
    0x0Bu8 => "Instruction TLB: 4 MByte pages, 4-way set associative, 4 entries",
    0x0Cu8 => "1st-level data cache: 16 KBytes, 4-way set associative, 32 byte line size",
    0x0Du8 => "1st-level data cache: 16 KBytes, 4-way set associative, 64 byte line size",
    0x0Eu8 => "1st-level data cache: 24 KBytes, 6-way set associative, 64 byte line size",
    0x1Du8 => "2nd-level cache: 128 KBytes, 2-way set associative, 64 byte line size",
    0x21u8 => "2nd-level cache: 256 KBytes, 8-way set associative, 64 byte line size",
    0x22u8 => "3rd-level cache: 512 KBytes, 4-way set associative, 64 byte line size, 2 lines per sector",
    0x23u8 => "3rd-level cache: 1 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector",
    0x24u8 => "2nd-level cache: 1 MBytes, 16-way set associative, 64 byte line size",
    0x25u8 => "3rd-level cache: 2 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector",
    0x29u8 => "3rd-level cache: 4 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector",
    0x2Cu8 => "1st-level data cache: 32 KBytes, 8-way set associative, 64 byte line size",
    0x30u8 => "1st-level instruction cache: 32 KBytes, 8-way set associative, 64 byte line size",
    0x40u8 => "No 2nd-level cache or, if processor contains a valid 2nd-level cache, no 3rd-level cache",
    0x41u8 => "2nd-level cache: 128 KBytes, 4-way set associative, 32 byte line size",
    0x42u8 => "2nd-level cache: 256 KBytes, 4-way set associative, 32 byte line size",
    0x43u8 => "2nd-level cache: 512 KBytes, 4-way set associative, 32 byte line size",
    0x44u8 => "2nd-level cache: 1 MByte, 4-way set associative, 32 byte line size",
    0x45u8 => "2nd-level cache: 2 MByte, 4-way set associative, 32 byte line size",
    0x46u8 => "3rd-level cache: 4 MByte, 4-way set associative, 64 byte line size",
    0x47u8 => "3rd-level cache: 8 MByte, 8-way set associative, 64 byte line size",
    0x48u8 => "2nd-level cache: 3MByte, 12-way set associative, 64 byte line size",
    0x49u8 => "3rd-level cache: 4MB, 16-way set associative, 64-byte line size (Intel Xeon processor MP, Family 0FH, Model 06H);\n2nd-level cache: 4 MByte, 16-way set associative, 64 byte line size",
    0x4Au8 => "3rd-level cache: 6MByte, 12-way set associative, 64 byte line size",
    0x4Bu8 => "3rd-level cache: 8MByte, 16-way set associative, 64 byte line size",
    0x4Cu8 => "3rd-level cache: 12MByte, 12-way set associative, 64 byte line size",
    0x4Du8 => "3rd-level cache: 16MByte, 16-way set associative, 64 byte line size",
    0x4Eu8 => "2nd-level cache: 6MByte, 24-way set associative, 64 byte line size",
    0x4Fu8 => "Instruction TLB: 4 KByte pages, 32 entries",
    0x50u8 => "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 64 entries",
    0x51u8 => "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 128 entries",
    0x52u8 => "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 256 entries",
    0x55u8 => "Instruction TLB: 2-MByte or 4-MByte pages, fully associative, 7 entries",
    0x56u8 => "Data TLB0: 4 MByte pages, 4-way set associative, 16 entries",
    0x57u8 => "Data TLB0: 4 KByte pages, 4-way associative, 16 entries",
    0x59u8 => "Data TLB0: 4 KByte pages, fully associative, 16 entries",
    0x5Au8 => "Data TLB0: 2 MByte or 4 MByte pages, 4-way set associative, 32 entries",
    0x5Bu8 => "Data TLB: 4 KByte and 4 MByte pages, 64 entries",
    0x5Cu8 => "Data TLB: 4 KByte and 4 MByte pages, 128 entries",
    0x5Du8 => "Data TLB: 4 KByte and 4 MByte pages, 256 entries",
    0x60u8 => "1st-level data cache: 16 KByte, 8-way set associative, 64 byte line size",
    0x61u8 => "Instruction TLB: 4 KByte pages, fully associative, 48 entries",
    0x63u8 => "Data TLB: 2 MByte or 4 MByte pages, 4-way set associative, 32 entries and a separate array with 1 GByte pages, 4-way set associative, 4 entries",
    0x64u8 => "Data TLB: 4 KByte pages, 4-way set associative, 512 entries",
    0x66u8 => "1st-level data cache: 8 KByte, 4-way set associative, 64 byte line size",
    0x67u8 => "1st-level data cache: 16 KByte, 4-way set associative, 64 byte line size",
    0x68u8 => "1st-level data cache: 32 KByte, 4-way set associative, 64 byte line size",
    0x6Au8 => "uTLB: 4 KByte pages, 8-way set associative, 64 entries",
    0x6Bu8 => "DTLB: 4 KByte pages, 8-way set associative, 256 entries",
    0x6Cu8 => "DTLB: 2M/4M pages, 8-way set associative, 128 entries",
    0x6Du8 => "DTLB: 1 GByte pages, fully associative, 16 entries",
    0x70u8 => "Trace cache: 12 K-μop, 8-way set associative",
    0x71u8 => "Trace cache: 16 K-μop, 8-way set associative",
    0x72u8 => "Trace cache: 32 K-μop, 8-way set associative",
    0x76u8 => "Instruction TLB: 2M/4M pages, fully associative, 8 entries",
    0x78u8 => "2nd-level cache: 1 MByte, 4-way set associative, 64byte line size",
    0x79u8 => "2nd-level cache: 128 KByte, 8-way set associative, 64 byte line size, 2 lines per sector",
    0x7Au8 => "2nd-level cache: 256 KByte, 8-way set associative, 64 byte line size, 2 lines per sector",
    0x7Bu8 => "2nd-level cache: 512 KByte, 8-way set associative, 64 byte line size, 2 lines per sector",
    0x7Cu8 => "2nd-level cache: 1 MByte, 8-way set associative, 64 byte line size, 2 lines per sector",
    0x7Du8 => "2nd-level cache: 2 MByte, 8-way set associative, 64byte line size",
    0x7Fu8 => "2nd-level cache: 512 KByte, 2-way set associative, 64-byte line size",
    0x80u8 => "2nd-level cache: 512 KByte, 8-way set associative, 64-byte line size",
    0x82u8 => "2nd-level cache: 256 KByte, 8-way set associative, 32 byte line size",
    0x83u8 => "2nd-level cache: 512 KByte, 8-way set associative, 32 byte line size",
    0x84u8 => "2nd-level cache: 1 MByte, 8-way set associative, 32 byte line size",
    0x85u8 => "2nd-level cache: 2 MByte, 8-way set associative, 32 byte line size",
    0x86u8 => "2nd-level cache: 512 KByte, 4-way set associative, 64 byte line size",
    0x87u8 => "2nd-level cache: 1 MByte, 8-way set associative, 64 byte line size",
    0xA0u8 => "DTLB: 4k pages, fully associative, 32 entries",
    0xB0u8 => "Instruction TLB: 4 KByte pages, 4-way set associative, 128 entries",
    0xB1u8 => "Instruction TLB: 2M pages, 4-way, 8 entries or 4M pages, 4-way, 4 entries",
    0xB2u8 => "Instruction TLB: 4KByte pages, 4-way set associative, 64 entries",
    0xB3u8 => "Data TLB: 4 KByte pages, 4-way set associative, 128 entries",
    0xB4u8 => "Data TLB1: 4 KByte pages, 4-way associative, 256 entries",
    0xB5u8 => "Instruction TLB: 4KByte pages, 8-way set associative, 64 entries",
    0xB6u8 => "Instruction TLB: 4KByte pages, 8-way set associative, 128 entries",
    0xBAu8 => "Data TLB1: 4 KByte pages, 4-way associative, 64 entries",
    0xC0u8 => "Data TLB: 4 KByte and 4 MByte pages, 4-way associative, 8 entries",
    0xC1u8 => "Shared 2nd-Level TLB: 4 KByte / 2 MByte pages, 8-way associative, 1024 entries",
    0xC2u8 => "DTLB: 4 KByte/2 MByte pages, 4-way associative, 16 entries",
    0xC3u8 => "Shared 2nd-Level TLB: 4 KByte / 2 MByte pages, 6-way associative, 1536 entries. Also 1GBbyte pages, 4-way, 16 entries.",
    0xC4u8 => "DTLB: 2M/4M Byte pages, 4-way associative, 32 entries",
    0xCAu8 => "Shared 2nd-Level TLB: 4 KByte pages, 4-way associative, 512 entries",
    0xD0u8 => "3rd-level cache: 512 KByte, 4-way set associative, 64 byte line size",
    0xD1u8 => "3rd-level cache: 1 MByte, 4-way set associative, 64 byte line size",
    0xD2u8 => "3rd-level cache: 2 MByte, 4-way set associative, 64 byte line size",
    0xD6u8 => "3rd-level cache: 1 MByte, 8-way set associative, 64 byte line size",
    0xD7u8 => "3rd-level cache: 2 MByte, 8-way set associative, 64 byte line size",
    0xD8u8 => "3rd-level cache: 4 MByte, 8-way set associative, 64 byte line size",
    0xDCu8 => "3rd-level cache: 1.5 MByte, 12-way set associative, 64 byte line size",
    0xDDu8 => "3rd-level cache: 3 MByte, 12-way set associative, 64 byte line size",
    0xDEu8 => "3rd-level cache: 6 MByte, 12-way set associative, 64 byte line size",
    0xE2u8 => "3rd-level cache: 2 MByte, 16-way set associative, 64 byte line size",
    0xE3u8 => "3rd-level cache: 4 MByte, 16-way set associative, 64 byte line size",
    0xE4u8 => "3rd-level cache: 8 MByte, 16-way set associative, 64 byte line size",
    0xEAu8 => "3rd-level cache: 12MByte, 24-way set associative, 64 byte line size",
    0xEBu8 => "3rd-level cache: 18MByte, 24-way set associative, 64 byte line size",
    0xECu8 => "3rd-level cache: 24MByte, 24-way set associative, 64 byte line size",
    0xF0u8 => "64-Byte prefetching",
    0xF1u8 => "128-Byte prefetching",
    0xFEu8 => "CPUID leaf 2 does not report TLB descriptor information; use CPUID leaf 18H to query TLB and other address translation parameters.",
    0xFFu8 => "CPUID leaf 2 does not report cache descriptor information, use CPUID leaf 4 to query cache parameters"
};

#[derive(Debug)]
pub struct TlbCachePrefetchInfomation {
    eax: [u8; 4],
    ebx: [u8; 4],
    ecx: [u8; 4],
    edx: [u8; 4],
}
impl fmt::Display for TlbCachePrefetchInfomation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a: [&'static str; 16] = self.into();
        write!(f, "{:#?}", a)
    }
}
// - The least-signficant-byte of eax always returns 01h.
// - The most significant bit indicates whether the register contains valid infomation (TODO Does
//   this mean we only have 3 descriptors per register?)
impl From<&TlbCachePrefetchInfomation> for [&'static str; 16] {
    fn from(this: &TlbCachePrefetchInfomation) -> Self {
        [
            KEYWORDS.get(&this.eax[0]).unwrap(),
            KEYWORDS.get(&this.eax[1]).unwrap(),
            KEYWORDS.get(&this.eax[2]).unwrap(),
            KEYWORDS.get(&this.eax[3]).unwrap(),
            KEYWORDS.get(&this.ebx[0]).unwrap(),
            KEYWORDS.get(&this.ebx[1]).unwrap(),
            KEYWORDS.get(&this.ebx[2]).unwrap(),
            KEYWORDS.get(&this.ebx[3]).unwrap(),
            KEYWORDS.get(&this.ecx[0]).unwrap(),
            KEYWORDS.get(&this.ecx[1]).unwrap(),
            KEYWORDS.get(&this.ecx[2]).unwrap(),
            KEYWORDS.get(&this.ecx[3]).unwrap(),
            KEYWORDS.get(&this.edx[0]).unwrap(),
            KEYWORDS.get(&this.edx[1]).unwrap(),
            KEYWORDS.get(&this.edx[2]).unwrap(),
            KEYWORDS.get(&this.edx[3]).unwrap(),
        ]
    }
}
impl From<(u32, u32, u32, u32)> for TlbCachePrefetchInfomation {
    fn from((eax, ebx, ecx, edx): (u32, u32, u32, u32)) -> Self {
        Self {
            eax: eax.to_ne_bytes(),
            ebx: ebx.to_ne_bytes(),
            ecx: ecx.to_ne_bytes(),
            edx: edx.to_ne_bytes(),
        }
    }
}
// -------------------------------------------------------------------------------------------------
// Leaf 1.0
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf1Eax, u32, {
    stepping_id, 0..4,
    model, 4..8,
    family_id, 8..12,
    processor_type, 12..14,
    extended_model_id: 16..20,
    extended_family_id: 20..28,
});
#[rustfmt::skip]
bitfield!(Leaf1Ebx, u32, {
    brand_index, 0..8,
    clflush, 8..16,
    // maximum_number_addressable_ids_for_logical_processors_in_this_physical_package
    max_addressable_logical_processor_ids: 16..24,
    initial_apic_id: 24..32,
});
#[rustfmt::skip]
bitfield!(Leaf1Ecx, u32, {
    sse3, 0, pclmulqdq, 1, dtes64, 2, monitor, 3, ds_cpl, 4, vmx, 5, smx, 6, eist, 7, tm2, 8,
    ssse3, 9, cnxt_id, 10, sdbg, 11, fma, 12, cmpxchg16b, 13, xtpr_update_control, 14, pdcm, 15,
    // Reserved
    pcid, 17, dca, 18, sse4_1, 19, sse4_2, 20, x2apic, 21, movbe, 22, popcnt, 23, tsc_deadline, 24,
    aesni, 25, xsave, 26, osxsave, 27, avx, 28, f16c, 29, rdrand, 30,
    // Not used
});
#[rustfmt::skip]
bitfield!(Leaf1Edx, u32, {
    fpu: 0, vme: 1, de: 2, pse: 3, tsc: 4, msr: 5, pae: 6, mce: 7, cx8: 8, apic: 9,
    // Reserved
    sep: 11, mtrr: 12, pge: 13, mca: 14, cmov: 15, pat: 16, pse3_36: 17, psn: 18, clfsh: 19,
    // Reserved
    ds: 21, acpi: 22, mmx: 23, fxsr: 24, sse: 25, sse2: 26, ss: 27, htt: 28, tm: 29,
    // Reserved
    pbe: 31,
});

// -------------------------------------------------------------------------------------------------
// Leaf 4
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf4Eax, u32, {
    // Cache Type Field.
    cache_type_field: 0..5,
    // Cache Level (starts at 1).
    cache_level: 5..8,
    // Self Initializing cache level (does not need SW initialization).
    sicl: 8,
    // Fully Associative cache.
    fac: 9,
    // Reserved 10..14
    // Maximum number of addressable IDs for logical processors sharing this cache.
    // - Add one to the return value to get the result.
    // - The nearest power-of-2 integer that is not smaller than (1 + EAX[25:14]) is the number of 
    //   unique initial APIC IDs reserved for addressing different logical processors sharing this 
    //   cache.
    max_num_addressable_ids_for_logical_processors_sharing_this_cache: 14..26,
    // Maximum number of addressable IDs for processor cores in the physical package.
    // - Add one to the return value to get the result.
    // - The nearest power-of-2 integer that is not smaller than (1 + EAX[31:26]) is the number of
    //   unique Core_IDs reserved for addressing different processor cores in a physical package. 
    //   Core ID is a subset of bits of the initial APIC ID.
    // - The returned value is constant for valid initial values in ECX. Valid ECX values start from 0.
    max_num_addressable_ids_for_processor_cores_in_physical_package: 26..32,
});
#[rustfmt::skip]
bitfield!(Leaf4Ebx, u32, {
    // L = System Coherency Line Size.
    // - Add one to the return value to get the result.
    system_coherency_line_size: 0..12,
    // P = Physical Line partitions.
    // - Add one to the return value to get the result.
    physical_line_partitions: 12..22,
    // W = Ways of associativity.
    // - Add one to the return value to get the result.
    ways_of_associativity: 22..32
});
#[rustfmt::skip]
bitfield!(Leaf4Ecx, u32, {
    // S = Number of Sets.
    // - Add one to the return value to get the result.
    number_of_sets: 0..32,
});
#[rustfmt::skip]
bitfield!(Leaf4Edx, u32, {
    // Write-Back Invalidate/Invalidate.
    // - 0 = WBINVD/INVD from threads sharing this cache acts upon lower level caches for threads sharing this cache.
    // - 1 = WBINVD/INVD is not guaranteed to act upon lower level caches of non-originating threads sharing this cache.
    write_back_invalidate: 0,
    // Cache Inclusiveness.
    // - 0 = Cache is not inclusive of lower cache levels.
    // - 1 = Cache is inclusive of lower cache levels.
    cache_inclusiveness: 1,
    // Complex Cache Indexing.
    // - 0 = Direct mapped cache.
    // - 1 = A complex function is used to index the cache, potentially using all address bits.
    complex_cache_indexing: 2
});
// -------------------------------------------------------------------------------------------------
// Leaf 5
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf5Eax, u32, {
    // Smallest monitor-line size in bytes (default is processor's monitor granularity).
    smallest_monitor_line_size: 0..16,
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf5Ebx, u32, {
    // Largest monitor-line size in bytes (default is processor's monitor granularity).
    largest_monitor_line_size: 0..16,
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf5Ecx, u32, {
    // Enumeration of Monitor-Mwait extensions (beyond EAX and EBX registers) supported.
    enum_monitor_mwait_ext: 0,
    // Supports treating interrupts as break-event for MWAIT, even when interrupts disabled.
    support_treating_interrupts_as_break_events_for_mwait: 1,
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf5Edx, u32, {
    // *The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, not ACPI Cstates.

    // Number of C0* sub C-states supported using MWAIT.
    c0_states: 0..4,
    // Number of C1* sub C-states supported using MWAIT.
    c1_states: 4..8,
    // Number of C2* sub C-states supported using MWAIT.
    c2_states: 8..12,
    // Number of C3* sub C-states supported using MWAIT.
    c3_states: 12..16,
    // Number of C4* sub C-states supported using MWAIT.
    c4_states: 16..20,
    // Number of C5* sub C-states supported using MWAIT.
    c5_states: 20..24,
    // Number of C6* sub C-states supported using MWAIT.
    c6_states: 24..28,
    // Number of C7* sub C-states supported using MWAIT.
    c7_states: 28..32
});
// -------------------------------------------------------------------------------------------------
// Leaf 6
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf6Eax, u32, {
    // Digital temperature sensor is supported if set.
    digital_temperature_sensor: 0,
    // Intel Turbo Boost Technology available (see description of IA32_MISC_ENABLE[38]).
    intel_turbo_boost_technology: 1,
    arat: 2,
    // Reserved
    pln: 4,
    ecmd: 5,
    ptm: 6,
    hwp: 7,
    hwp_notification: 8,
    hwp_activity_window: 9,
    hwp_energy_performance: 10,
    hwp_package_level_request: 11,
    // Reserved
    hdc: 13,
    intel_turbo_boost_max_technology_3: 14,
    hwp_capabilities: 15,
    hwp_peci_override: 16,
    flexible_hwp: 17,
    // Fast access mode for the IA32_HWP_REQUEST MSR is supported if set.
    fast_access_mode_for_i32_hwp_request_msr: 18,
    hw_feedback: 19,
    // Ignoring Idle Logical Processor HWP request is supported if set.
    iilp_hwp_r; 20,
    // Reserved 21..=22
    intel_thread_director: 23,
    // Reserved 24..=31
    
});
#[rustfmt::skip]
bitfield!(Leaf6Ebx, u32, {
    // Number of Interrupt Thresholds in Digital Thermal Sensor.
    number_of_interrupt_thresholds_in_digital_thermal_sensor: 0..4,
    // Reserved 4..=31
});
#[rustfmt::skip]
bitfield!(Leaf6Ecx, u32, {
    // Hardware Coordination Feedback Capability (Presence of IA32_MPERF and IA32_APERF). The 
    // capability to provide a measure of delivered processor performance (since last reset of the 
    // counters), as a percentage of the expected processor performance when running at the TSC
    // frequency.
    hardware_coordination_feedback_capability: 0,
    // Reserved 1..=2
    // The processor supports performance-energy bias preference if CPUID.06H:ECX.SETBH[bit 3] is 
    // set and it also implies the presence of a new architectural MSR called IA32_ENERGY_PERF_BIAS
    // (1B0H).
    performance_energy_bias: 3,
    // Reserved 04..=07
    // Number of Intel® Thread Director classes supported by the processor. Information for that
    // many classes is written into the Intel Thread Director Table by the hardware.
    intel_thread_director_classes: 8..16,
    // Reserved 16..=31
});
#[rustfmt::skip]
bitfield!(Leaf6Edx, u32, {
    // Bitmap of supported hardware feedback interface capabilities.
    // - 0 = When set to 1, indicates support for performance capability reporting.
    // - 1 = When set to 1, indicates support for energy efficiency capability reporting.
    // - 2-7 = Reserved
    bitmap_hardware_feedback_interface_capabilities: 0..8,
    // Enumerates the size of the hardware feedback interface structure in number of 4 KB pages; add
    // one to the return value to get the result.
    enum_hardware_feedback_interface_4k: 8..12,
    // Index (starting at 0) of this logical processor's row in the hardware feedback interface
    // structure. Note that on some parts the index may be same for multiple logical processors. On
    // some parts the indices may not be contiguous, i.e., there may be unused rows in the hardware
    // feedback interface structure.
    index: 16..32
});
// -------------------------------------------------------------------------------------------------
// Leaf 7
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf7Subleaf0Eax, u32, {
    // Reports the maximum input value for supported leaf 7 sub-leaves.
    max_input_value_subleaf: 0..32
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf0Ebx, u32, {
    fsgsbase: 0,
    ia32_tsc_adjust_msr: 1,
    sgx: 2,
    bmi1: 3,
    hle: 4,
    avx2: 5,
    fdp_excptn_only: 6,
    smep: 7,
    bmi2: 8,
    suports_enhanced_rep_movsb_stosb: 9,
    invpcid: 10,
    rtm: 11,
    rdt_m: 12,
    deprecates_fpu_cs_and_fpu_ds: 13,
    mpx: 14,
    rdt_t: 15,
    avx512f: 16,
    avx512dq: 17,
    rdseed: 18,
    adx: 19,
    smap: 20,
    avx512_ifma: 21,
    // Reserved
    clfushopt: 23,
    clwb: 24,
    intel_processor_trace: 25,
    avx512pf: 26,
    avx512er: 27,
    avx512cd: 28,
    sha: 29,
    avx512bw: 30,
    avx512vl: 31
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf0Ecx, u32, {
    // Docs note 'PREFETCHWT1. (Intel® Xeon Phi™ only.)'
    // prefetchwt1: 0,
    avx512_vbmi: 1,
    umip: 2,
    pku: 3,
    ospke: 4,
    waitpkg: 5,
    avx512_vbmi2: 6,
    cet_ss: 7,
    gfni: 8,
    vaes: 9,
    vpclmulqdq: 10,
    avx512_vnni: 11,
    avx512_bitalg: 12,
    tme_en; 13,
    avx512_vpopcntdq: 14,
    // Reserved
    la57: 16,
    value_of_mawau: 17..22,
    rdpid_and_ia32_tsc_aux: 22,
    kl: 23,
    // Reserved
    cldemote: 25,
    // Reserved
    movdiri: 27,
    movdiri64b: 28,
    // Reserved
    sgx_lc: 30,
    pks: 31
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf0Edx, u32, {
    // Reserved
    // Docs note '(Intel® Xeon Phi™ only.)'
    // avx512_4vnniw: 2
    // Docs note '(Intel® Xeon Phi™ only.)'
    // avx512_4fmaps: 3
    fast_short_rep_mov: 4,
    // Reserved 5..=7
    avx512_vp2intersect: 8,
    // Reserved
    md_clear: 10,
    // Reserved
    serialize:  11..14,
    hydrid: 15,
    // Reserved 16..=17
    pconfig: 18,
    // Reserved
    cet_ibt: 19,
    // Reserved 21..=25
    // Enumerates support for indirect branch restricted speculation (IBRS) and the indirect branch
    // predictor barrier (IBPB). Processors that set this bit support the IA32_SPEC_CTRL MSR and the
    // IA32_PRED_CMD MSR. They allow software to set IA32_SPEC_CTRL[0] (IBRS) and IA32_PRED_CMD[0]
    // (IBPB).
    ibrs_ibpb_enum: 26,
    // Enumerates support for single thread indirect branch predictors (STIBP). Processors that set
    // this bit support the IA32_SPEC_CTRL MSR. They allow software to set IA32_SPEC_CTRL[1] 
    // (STIBP).
    stibp_enum: 27,
    // Enumerates support for L1D_FLUSH. Processors that set this bit support the IA32_FLUSH_CMD 
    // MSR. They allow software to set IA32_FLUSH_CMD[0] (L1D_FLUSH).
    l1d_flush_enum: 28,
    // Enumerates support for the IA32_ARCH_CAPABILITIES MSR.
    ia32_arch_capabilities_msr_enum: 29,
    // Enumerates support for the IA32_CORE_CAPABILITIES MSR.
    ia32_core_capabilities_msr_enum: 30,
    // Enumerates support for Speculative Store Bypass Disable (SSBD). Processors that set this bit
    // support the IA32_SPEC_CTRL MSR. They allow software to set IA32_SPEC_CTRL[2] (SSBD).
    ssbd_enum: 31
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf1Eax, u32, {
    // Reserved 0..=3
    avx_vnni: 4,
    avx512_bf16: 5,
    // Reserved 6..=9
    fast_zero_length_rep_movsh: 10,
    fast_short_rep_stosb: 11,
    fast_short_rep_cmpsb_rep_scasb: 12,
    // Reserved 13..=21
    hreset: 22,
    // Reserved 23..=31
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf1Ebx, u32, {
    ia32_ppin_and_ia32_ppin_ctl_msrs_enum: 0
    // Reserved 1..=31
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf1Ecx, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf1Edx, u32, {
    // Reserved
});
// -------------------------------------------------------------------------------------------------
// Leaf 9
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf9Eax, u32, {
    // Value of bits [31:0] of IA32_PLATFORM_DCA_CAP MSR (address 1F8H).
    ia32_paltform_dca_cap_msr: 0..32
});
#[rustfmt::skip]
bitfield!(Leaf9Ebx, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf9Ecx, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf9Edx, u32, {
    // Reserved
});
// -------------------------------------------------------------------------------------------------
// Leaf A
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(LeafAEax, u32, {
    // Version ID of architectural performance monitoring.
    version_id_of_architectural_performance_monitoring: 0..8,
    // Number of general-purpose performance monitoring counter per logical processor.
    num_perf_monitor_counter_per_logical_processor: 8..16,
    // Bit width of general-purpose, performance monitoring counter.
    bot_width_perf_monitor_counter: 16..24,
    // Length of EBX bit vector to enumerate architectural performance monitoring events. 
    // Architectural event x is supported if EBX[x]=0 && EAX[31:24]>x.
    len_ebx_bit_vec: 24..32
});
#[rustfmt::skip]
bitfield!(LeafAEbx, u32, {
    // Core cycle event not available if 1 or if EAX[31:24]<1.
    core_cycle_event: 0,
    // Instruction retired event not available if 1 or if EAX[31:24]<2.
    instruction_retired_event: 1,
    // Reference cycles event not available if 1 or if EAX[31:24]<3.
    reference_cycles_event: 2,
    // Last-level cache reference event not available if 1 or if EAX[31:24]<4.
    last_level_cache_reference_event: 3,
    // Last-level cache misses event not available if 1 or if EAX[31:24]<5.
    last_level_cache_misses_event: 4,
    // Branch instruction retired event not available if 1 or if EAX[31:24]<6.
    branch_instruction_retired_event: 5,
    // Branch mispredict retired event not available if 1 or if EAX[31:24]<7.
    branch_mispredict_retired_event: 6,
    // Top-down slots event not available if 1 or if EAX[31:24]<8.
    top_down_slots_event: 7,
    // Reserved 8..=31
});
#[rustfmt::skip]
bitfield!(LeafAEcx, u32, {
    // Supported fixed counters bit mask. Fixed-function performance counter 'i' is supported if bit
    // ‘i’ is 1 (first counter index starts at zero). It is recommended to use the following logic 
    // to determine if a Fixed Counter is supported: 
    // FxCtr[i]_is_supported := ECX[i] || (EDX[4:0] > i);
    supported_fixed_counters_bit_mask: 0..32
});
#[rustfmt::skip]
bitfield!(LeafAEdx, u32, {
    // Number of contiguous fixed-function performance counters starting from 0 (if Version ID >1).
    contigous_fixed_function_performance_counter: 0..5,
    // Bit width of fixed-function performance counters (if Version ID > 1).
    bit_width_of_fixed_function_performnace_counter: 5..13,
    // Reserved 13..=14
    anythread_deprecation: 15
    // Reserved 16..=31
});
// -------------------------------------------------------------------------------------------------
// Leaf B
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(LeafBEax, u32, {
    // Number of bits to shift right on x2APIC ID to get a unique topology ID of the next level 
    // type*. All logical processors with the same next level ID share current level.
    bit_shifts_right_2x_apic_id_unique_topology_id: 0..5
});
#[rustfmt::skip]
bitfield!(LeafBEbx, u32, {
    // Number of logical processors at this level type. The number reflects configuration as shipped
    // by Intel.
    //
    // Software must not use EBX[15:0] to enumerate processor topology of the system. This value in 
    // this field (EBX[15:0]) is only intended for display/diagnostic purposes. The actual number of 
    // logical processors available to BIOS/OS/Applications may be different from the value of 
    // EBX[15:0], depending on software and platform hardware configurations.
    logical_processors: 0..16
});
#[rustfmt::skip]
bitfield!(LeafBEcx, u32, {
    level_number: 0..8,
    // If an input value n in ECX returns the invalid level-type of 0 in ECX[15:8], other input 
    // values with ECX>n also return 0 in ECX[15:8].
    //
    // The value of the “level type” field is not related to level numbers in any way, higher 
    // “level type” values do not mean higher levels. Level type field has the following encoding:
    // - 0: Invalid.
    // - 1: SMT.
    // - 2: Core.
    // - 3-255: Reserved.
    level_type: 8..16
    // Reserved 16..=31
});
#[rustfmt::skip]
bitfield!(LeafBEdx, u32, {
    // x2APIC ID the current logical processor.
    x2_apic_id_current_logical_processor: 0..32
});
// -------------------------------------------------------------------------------------------------
// Leaf D
// -------------------------------------------------------------------------------------------------
// Leaf 0
#[rustfmt::skip]
bitfield!(LeafDSubleaf0Eax, u32, {
    // Bits 31 - 00: Reports the supported bits of the lower 32 bits of XCR0. XCR0[n] can be set to 
    // 1 only if EAX[n] is 1.
    x86_state: 0,
    sse_state: 1,
    avx_state: 2,
    mpx_state: 3..5,
    avx512_state: 5..8,
    used_for_ia32_xss: 8,
    pkru_state: 9,
    // Reserved 10..=12
    used_for_ia32_xss_1: 13,
    // Reserved 14..=15
    used_for_ia32_xss_2: 16,
    // Reserved 17..=31
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf0Ebx, u32, {
    // Maximum size in bytes required by enabled features.
    maximum_size: 0..32
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf0Ecx, u32, {
    // Maximum size in bytes required by all supported features 
    // (`LeafDSubleaf0Ecx::maximum_size() >= LeafDSubleaf0Ebx::maximum_size()`).
    maximum_size: 0..32
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf0Edx, u32, {
    supported_bits_of_upper_32_bits_of_xcro: 0..32
});
// Leaf 1
#[rustfmt::skip]
bitfield!(LeafDSubleaf1Eax, u32, {
    xsaveopt_available: 0,
    xsavec_compacted_xrstor: 1,
    xgetbv: 2,
    xsaves_xrstors_ia32_xss: 3,
    // Reserved 0..32

});
#[rustfmt::skip]
bitfield!(LeafDSubleaf1Ebx, u32, {
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf1Ecx, u32, {
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf1Edx, u32, {
});
// -------------------------------------------------------------------------------------------------
// struct definition
// -------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Leaf<A, B, C, D> {
    pub eax: A,
    pub ebx: B,
    pub ecx: C,
    pub edx: D,
}
impl<A, B, C, D> From<(A, B, C, D)> for Leaf<A, B, C, D> {
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        Leaf {
            eax: a,
            ebx: b,
            ecx: c,
            edx: d,
        }
    }
}
type Leaf0 = Leaf<u32, FixedString<4>, FixedString<4>, FixedString<4>>;
type Leaf1 = Leaf<Leaf1Eax, Leaf1Ebx, Leaf1Ecx, Leaf1Edx>;
type Leaf2 = TlbCachePrefetchInfomation;
type Leaf4 = Leaf<Leaf4Eax, Leaf4Ebx, Leaf4Ecx, Leaf4Edx>;
type Leaf5 = Leaf<Leaf5Eax, Leaf5Ebx, Leaf5Ecx, Leaf5Edx>;
type Leaf6 = Leaf<Leaf6Eax, Leaf6Ebx, Leaf6Ecx, Leaf6Edx>;
type Leaf7 = (Leaf7Subleaf0, Option<Leaf7Subleaf1>);
type Leaf7Subleaf0 = Leaf<Leaf7Subleaf0Eax, Leaf7Subleaf0Ebx, Leaf7Subleaf0Ecx, Leaf7Subleaf0Edx>;
type Leaf7Subleaf1 = Leaf<Leaf7Subleaf1Eax, Leaf7Subleaf1Ebx, Leaf7Subleaf1Ecx, Leaf7Subleaf1Edx>;
type Leaf9 = Leaf<Leaf9Eax, Leaf9Ebx, Leaf9Ecx, Leaf9Edx>;
type LeafA = Leaf<LeafAEax, LeafAEbx, LeafAEcx, LeafAEdx>;
type LeafB = Leaf<LeafBEax, LeafBEbx, LeafBEcx, LeafBEdx>;
// type LeafD = (LeafDSubleaf0,LeafDSubleaf1,Vec<LeafDSubleafGt1>);
// type LeafDSubleaf0 = Leaf<LeafDSubleaf0Eax,LeafDSubleaf0Ebx,LeafDSubleaf0Ecx,LeafDSubleaf0Edx>;
// type LeafDSubleaf1 = Leaf<LeafDSubleaf1Eax,LeafDSubleaf1Ebx,LeafDSubleaf1Ecx,LeafDSubleaf1Edx>;

impl Leaf0 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        self.eax >= other.eax
            && self.ebx == other.ebx
            && self.ecx == other.ecx
            && self.edx == other.edx
    }
}
impl Leaf1 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        // TODO Check not Pentium III and not Intel Xeon Phi.
        // TODO Check ebx
        self.ecx.superset(&other.ecx) && self.edx.superset(&other.edx)
    }
}
impl Leaf2 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf4 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf5 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        self.eax.smallest_monitor_line_size() <= other.eax.smallest_monitor_line_size()
            && self.ebx.largest_monitor_line_size() >= other.ebx.largest_monitor_line_size()
            && self.ecx.superset(&other.ecx)
        // TODO edx
    }
}
impl Leaf6 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        self.eax.superset(&self.eax)
            && self
                .ebx
                .number_of_interrupt_thresholds_in_digital_thermal_sensor()
                >= other
                    .ebx
                    .number_of_interrupt_thresholds_in_digital_thermal_sensor()
        // TODO ecx
        // TODO edx
    }
}
impl Leaf7Subleaf0 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        self.eax.max_input_value_subleaf() >= other.eax.max_input_value_subleaf()
            && self.ebx.superset(&self.ebx)
            && self.ecx.superset(&self.ecx)
            && self.edx.superset(&self.edx)
    }
}
impl Leaf7Subleaf1 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        self.eax.superset(&self.eax) && self.ebx.superset(&self.ebx)
    }
}
impl Leaf9 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        // TODO Can we use `>=` here instead?
        self.eax.ia32_paltform_dca_cap_msr() == other.eax.ia32_paltform_dca_cap_msr()
    }
}
impl LeafA {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        // Do any of these feature affect program functionality or security?
        todo!()
    }
}
/// - Does not support Pentium III processor.
/// - bit 22 of `IA32_MISC_ENABLE` equals 0.
/// - Presuming the flag `CPUID leaf 2 does not report cache descriptor information, use CPUID leaf
///   4 to query cache parameters` is present in leaf 2.
/// - Does not support Intel® Xeon Phi™.
#[derive(Debug)]
pub struct IntelCpuid {
    /// Basic CPUID Information
    pub leaf_0: Leaf0,
    /// Basic CPUID Information
    pub leaf_1: Leaf1,
    /// Basic CPUID Information
    pub leaf_2: Leaf2,
    // 'leaf 3 is only used in 'Pentium III processor', we can ignore this by explicitly noting we
    // do not support it.' I beleive we can presume we are not running on Pentium III
    // processors.
    //
    // 'CPUID leaves above 2 and below 80000000H are visible only when IA32_MISC_ENABLE[bit 22] has
    // its default value of 0.' I beleive we can presume this is true.
    // leaf4
    /// Deterministic Cache Parameters Leaf
    pub leaf_4: Vec<Leaf4>,
    /// MONITOR/MWAIT Leaf
    pub leaf_5: Leaf5,
    /// Thermal and Power Management Leaf
    pub leaf_6: Leaf6,
    // Presuming leaf 7 subleaf 0 (eax 7, ecx 0) eax equals 1
    /// Structured Extended Feature Flags Enumeration Leaf (Output depends on ECX input value)
    pub leaf_7: Leaf7,
    /// Direct Cache Access Information Leaf
    pub leaf_9: Leaf9,
    /// Architectural Performance Monitoring Leaf
    pub leaf_a: LeafA,
    /// Extended Topology Enumeration Leaf
    pub leaf_b: Vec<LeafB>, 
    // /// Processor Extended State Enumeration Main Leaf
    // pub leaf_d: LeafD
}
impl IntelCpuid {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        self.leaf_0.supports(&other.leaf_0) &&
        self.leaf_1.supports(&other.leaf_1) &&
        self.leaf_2.supports(&other.leaf_2) &&
        // TODO leaf 4
        // self.leaf_4.supports(&other.leaf_4) &&
        self.leaf_5.supports(&other.leaf_5) &&
        self.leaf_6.supports(&other.leaf_6) &&
        self.leaf_7.0.supports(&other.leaf_7.0) &&
        // TODO leaf 7 subleaf 1
        self.leaf_9.supports(&other.leaf_9) &&
        self.leaf_a.supports(&other.leaf_a)
    }
}
impl From<RawCpuid> for IntelCpuid {
    #[allow(clippy::too_many_lines)]
    fn from(raw_cpuid: RawCpuid) -> Self {
        let mut leaf_4_offset = 3;
        let leaf_4 = {
            let mut vec = Vec::new();
            loop {
                leaf_4_offset += 1;
                if raw_cpuid[leaf_4_offset].eax == 0 {
                    break;
                }
                vec.push(Leaf4::from((
                    Leaf4Eax::from(raw_cpuid[leaf_4_offset].eax),
                    Leaf4Ebx::from(raw_cpuid[leaf_4_offset].ebx),
                    Leaf4Ecx::from(raw_cpuid[leaf_4_offset].ecx),
                    Leaf4Edx::from(raw_cpuid[leaf_4_offset].edx),
                )));
            }
            vec
        };
        println!(
            "Leaf7Subleaf0Eax::from(raw_cpuid[leaf_4_offset + 3].eax): {}",
            raw_cpuid[leaf_4_offset + 3].eax
        );
        // for i in 0..15 {
        //     println!("raw_cpuid[{}].ebx: {}",i,raw_cpuid[i].ebx);
        // }
        debug_assert_eq!(
            raw_cpuid[leaf_4_offset + 3].ebx,
            raw_cpuid.get(7, 0).unwrap().ebx
        );
        let leaf_7_offset = raw_cpuid[leaf_4_offset + 3].eax as usize;
        debug_assert!(leaf_7_offset == 0 || leaf_7_offset == 1);
        let leaf7_subleaves = if leaf_7_offset == 1 {
            Some(Leaf7Subleaf1::from((
                Leaf7Subleaf1Eax::from(raw_cpuid[leaf_4_offset + 3 + 1].eax),
                Leaf7Subleaf1Ebx::from(raw_cpuid[leaf_4_offset + 3 + 1].ebx),
                Leaf7Subleaf1Ecx::from(raw_cpuid[leaf_4_offset + 3 + 1].ecx),
                Leaf7Subleaf1Edx::from(raw_cpuid[leaf_4_offset + 3 + 1].edx),
            )))
        } else {
            None
        };
        let mut leaf_b_offset = leaf_4_offset + leaf_7_offset + 1;
        let leaf_b = {
            let mut vec = vec![LeafB::from((
                LeafBEax::from(raw_cpuid[leaf_b_offset].eax),
                LeafBEbx::from(raw_cpuid[leaf_b_offset].ebx),
                LeafBEcx::from(raw_cpuid[leaf_b_offset].ecx),
                LeafBEdx::from(raw_cpuid[leaf_b_offset].edx),
            ))];
            while *vec[vec.len() - 1].ecx.level_type() != 0u32 {
                leaf_b_offset += 1;
                vec.push(LeafB::from((
                    LeafBEax::from(raw_cpuid[leaf_b_offset].eax),
                    LeafBEbx::from(raw_cpuid[leaf_b_offset].ebx),
                    LeafBEcx::from(raw_cpuid[leaf_b_offset].ecx),
                    LeafBEdx::from(raw_cpuid[leaf_b_offset].edx),
                )));
            }
            vec
        };

        Self {
            leaf_0: Leaf0::from((
                raw_cpuid[0].eax,
                FixedString(raw_cpuid[0].ebx.to_ne_bytes()),
                FixedString(raw_cpuid[0].ecx.to_ne_bytes()),
                FixedString(raw_cpuid[0].edx.to_ne_bytes()),
            )),
            leaf_1: Leaf1::from((
                Leaf1Eax::from(raw_cpuid[1].eax),
                Leaf1Ebx::from(raw_cpuid[1].ebx),
                Leaf1Ecx::from(raw_cpuid[1].ecx),
                Leaf1Edx::from(raw_cpuid[1].edx),
            )),
            leaf_2: Leaf2::from((
                raw_cpuid[2].eax,
                raw_cpuid[2].ebx,
                raw_cpuid[2].ecx,
                raw_cpuid[2].edx,
            )),
            leaf_4,
            leaf_5: Leaf5::from((
                Leaf5Eax::from(raw_cpuid[leaf_4_offset + 1].eax),
                Leaf5Ebx::from(raw_cpuid[leaf_4_offset + 1].ebx),
                Leaf5Ecx::from(raw_cpuid[leaf_4_offset + 1].ecx),
                Leaf5Edx::from(raw_cpuid[leaf_4_offset + 1].edx),
            )),
            leaf_6: Leaf6::from((
                Leaf6Eax::from(raw_cpuid[leaf_4_offset + 2].eax),
                Leaf6Ebx::from(raw_cpuid[leaf_4_offset + 2].ebx),
                Leaf6Ecx::from(raw_cpuid[leaf_4_offset + 2].ecx),
                Leaf6Edx::from(raw_cpuid[leaf_4_offset + 2].edx),
            )),
            leaf_7: (
                Leaf7Subleaf0::from((
                    Leaf7Subleaf0Eax::from(raw_cpuid[leaf_4_offset + 3].eax),
                    Leaf7Subleaf0Ebx::from(raw_cpuid[leaf_4_offset + 3].ebx),
                    Leaf7Subleaf0Ecx::from(raw_cpuid[leaf_4_offset + 3].ecx),
                    Leaf7Subleaf0Edx::from(raw_cpuid[leaf_4_offset + 3].edx),
                )),
                leaf7_subleaves,
            ),
            leaf_9: Leaf9::from((
                Leaf9Eax::from(raw_cpuid[leaf_4_offset + leaf_7_offset + 5].eax),
                Leaf9Ebx::from(raw_cpuid[leaf_4_offset + leaf_7_offset + 5].ebx),
                Leaf9Ecx::from(raw_cpuid[leaf_4_offset + leaf_7_offset + 5].ecx),
                Leaf9Edx::from(raw_cpuid[leaf_4_offset + leaf_7_offset + 5].edx),
            )),
            leaf_a: LeafA::from((
                LeafAEax::from(raw_cpuid[leaf_4_offset + leaf_7_offset + 6].eax),
                LeafAEbx::from(raw_cpuid[leaf_4_offset + leaf_7_offset + 6].ebx),
                LeafAEcx::from(raw_cpuid[leaf_4_offset + leaf_7_offset + 6].ecx),
                LeafAEdx::from(raw_cpuid[leaf_4_offset + leaf_7_offset + 6].edx),
            )),
            leaf_b,
        }
    }
}
