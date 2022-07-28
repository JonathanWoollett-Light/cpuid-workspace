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
    /// Maximum Input Value for Basic CPUID Information.
    eax: [u8; 4],
    /// “Genu”
    ebx: [u8; 4],
    /// “ntel”
    ecx: [u8; 4],
    /// “ineI”
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
    /// Stepping ID
    stepping_id, 0..4,
    /// Model
    model, 4..8,
    /// Family ID
    family_id, 8..12,
    /// Processor Type
    processor_type, 12..14,
    /// Extended Model ID
    extended_model_id: 16..20,
    /// Extended Family ID
    extended_family_id: 20..28,
});
#[rustfmt::skip]
bitfield!(Leaf1Ebx, u32, {
    /// Brand Index.
    brand_index, 0..8,
    /// CLFLUSH line size (Value ∗ 8 = cache line size in bytes; used also by CLFLUSHOPT).
    clflush, 8..16,
    /// Maximum number of addressable IDs for logical processors in this physical package.
    ///
    /// The nearest power-of-2 integer that is not smaller than EBX[23:16] is the number of unique 
    /// initial APIC IDs reserved for addressing different logical processors in a physical package. 
    /// This field is only valid if CPUID.1.EDX.HTT[bit 28]= 1.
    max_addressable_logical_processor_ids: 16..24,
    /// Initial APIC ID.
    ///
    /// The 8-bit initial APIC ID in EBX[31:24] is replaced by the 32-bit x2APIC ID, available in 
    /// Leaf 0BH and Leaf 1FH.
    initial_apic_id: 24..32,
});
#[rustfmt::skip]
bitfield!(Leaf1Ecx, u32, {
    /// Streaming SIMD Extensions 3 (SSE3). A value of 1 indicates the processor supports this
    /// technology.
    sse3: 0, 
    /// PCLMULQDQ. A value of 1 indicates the processor supports the PCLMULQDQ instruction.
    pclmulqdq: 1,
    /// 64-bit DS Area. A value of 1 indicates the processor supports DS area using 64-bit layout.
    dtes64: 2,
    /// MONITOR/MWAIT. A value of 1 indicates the processor supports this feature.
    monitor: 3,
    /// CPL Qualified Debug Store. A value of 1 indicates the processor supports the extensions to 
    /// the Debug Store feature to allow for branch message storage qualified by CPL.
    ds_cpl: 4,
    /// Virtual Machine Extensions. A value of 1 indicates that the processor supports this 
    /// technology.
    vmx: 5,
    /// Safer Mode Extensions. A value of 1 indicates that the processor supports this technology. 
    /// See Chapter 6, “Safer Mode Extensions Reference”.
    smx: 6,
    /// Enhanced Intel SpeedStep® technology. A value of 1 indicates that the processor supports 
    /// this technology. 
    eist: 7,
    /// Thermal Monitor 2. A value of 1 indicates whether the processor supports this technology.
    tm2: 8,
    /// A value of 1 indicates the presence of the Supplemental Streaming SIMD Extensions 3 (SSSE3).
    /// A value of 0 indicates the instruction extensions are not present in the processor.
    ssse3: 9,
    /// L1 Context ID. A value of 1 indicates the L1 data cache mode can be set to either adaptive 
    /// mode or shared mode. A value of 0 indicates this feature is not supported. See definition of
    /// the IA32_MISC_ENABLE MSR Bit 24 (L1 Data Cache Context Mode) for details.
    cnxt_id: 10, 
    /// A value of 1 indicates the processor supports IA32_DEBUG_INTERFACE MSR for silicon debug.
    sdbg: 11,
    /// A value of 1 indicates the processor supports FMA extensions using YMM state.
    fma: 12, 
    /// CMPXCHG16B Available. A value of 1 indicates that the feature is available. See the 
    /// “CMPXCHG8B/CMPXCHG16B—Compare and Exchange Bytes” section in this chapter for a description.
    cmpxchg16b: 13, 
    /// xTPR Update Control. A value of 1 indicates that the processor supports changing 
    /// IA32_MISC_ENABLE[bit 23].
    xtpr_update_control: 14,
    /// Perfmon and Debug Capability: A value of 1 indicates the processor supports the performance
    /// and debug feature indication MSR IA32_PERF_CAPABILITIES.
    pdcm: 15,
    // Reserved
    /// Process-context identifiers. A value of 1 indicates that the processor supports PCIDs and 
    /// that software may set CR4.PCIDE to 1.
    pcid: 17,
    /// A value of 1 indicates the processor supports the ability to prefetch data from a memory 
    /// mapped device.
    dca: 18, 
    /// A value of 1 indicates that the processor supports SSE4.1.
    sse4_1: 19,
    /// A value of 1 indicates that the processor supports SSE4.2.
    sse4_2: 20,
    /// A value of 1 indicates that the processor supports x2APIC feature.
    x2apic: 21,
    /// A value of 1 indicates that the processor supports MOVBE instruction.
    movbe: 22, 
    /// A value of 1 indicates that the processor supports the POPCNT instruction.
    popcnt: 23,
    /// A value of 1 indicates that the processor’s local APIC timer supports one-shot operation 
    /// using a TSC deadline value.
    tsc_deadline: 24,
    /// A value of 1 indicates that the processor supports the AESNI instruction extensions.
    aesni: 25, 
    /// A value of 1 indicates that the processor supports the XSAVE/XRSTOR processor extended 
    /// states feature, the XSETBV/XGETBV instructions, and XCR0.
    xsave: 26,
    /// A value of 1 indicates that the OS has set CR4.OSXSAVE[bit 18] to enable XSETBV/XGETBV 
    /// instructions to access XCR0 and to support processor extended state management using 
    /// XSAVE/XRSTOR.
    osxsave: 27,
    /// A value of 1 indicates the processor supports the AVX instruction extensions.
    avx: 28, 
    /// A value of 1 indicates that processor supports 16-bit floating-point conversion instructions.
    f16c: 29, 
    /// A value of 1 indicates that processor supports RDRAND instruction.
    rdrand: 30,
    // Not used
    // TODO Should `Not used` be a flag?
});
#[rustfmt::skip]
bitfield!(Leaf1Edx, u32, {
    /// Floating Point Unit On-Chip. The processor contains an x87 FPU.
    fpu: 0, 
    /// Virtual 8086 Mode Enhancements. Virtual 8086 mode enhancements, including CR4.VME for 
    /// controlling the feature, CR4.PVI for protected mode virtual interrupts, software interrupt 
    /// indirection, expansion of the TSS with the software indirection bitmap, and EFLAGS.VIF and 
    /// EFLAGS.VIP flags.
    vme: 1,
    /// Debugging Extensions. Support for I/O breakpoints, including CR4.DE for controlling the 
    /// feature, and optional trapping of accesses to DR4 and DR5.
    de: 2, 
    /// Page Size Extension. Large pages of size 4 MByte are supported, including CR4.PSE for 
    /// controlling the feature, the defined dirty bit in PDE (Page Directory Entries), optional 
    /// reserved bit trapping in CR3, PDEs, and PTEs.
    pse: 3, 
    /// Time Stamp Counter. The RDTSC instruction is supported, including CR4.TSD for controlling 
    /// privilege.
    tsc: 4,
    /// Model Specific Registers RDMSR and WRMSR Instructions. The RDMSR and WRMSR instructions are 
    /// supported. Some of the MSRs are implementation dependent.
    msr: 5,
    /// Physical Address Extension. Physical addresses greater than 32 bits are supported: extended 
    /// page table entry formats, an extra level in the page translation tables is defined, 2-MByte 
    /// pages are supported instead of 4 Mbyte pages if PAE bit is 1.
    pae: 6, 
    /// Machine Check Exception. Exception 18 is defined for Machine Checks, including CR4.MCE for 
    /// controlling the feature. This feature does not define the model-specific implementations of 
    /// machine-check error logging, reporting, and processor shutdowns. Machine Check exception 
    /// handlers may have to depend on processor version to do model specific processing of the 
    /// exception, or test for the presence of the Machine Check feature.
    mce: 7,
    /// CMPXCHG8B Instruction. The compare-and-exchange 8 bytes (64 bits) instruction is supported 
    /// (implicitly locked and atomic).
    cx8: 8,
    /// APIC On-Chip. The processor contains an Advanced Programmable Interrupt Controller (APIC), 
    /// responding to memory mapped commands in the physical address range FFFE0000H to FFFE0FFFH 
    /// (by default - some processors permit the APIC to be relocated).
    apic: 9,
    // Reserved
    /// SYSENTER and SYSEXIT Instructions. The SYSENTER and SYSEXIT and associated MSRs are 
    /// supported.
    sep: 11, 
    /// Memory Type Range Registers. MTRRs are supported. The MTRRcap MSR contains feature bits that
    /// describe what memory types are supported, how many variable MTRRs are supported, and whether
    /// fixed MTRRs are supported.
    mtrr: 12,
    /// Page Global Bit. The global bit is supported in paging-structure entries that map a page, 
    /// indicating TLB entries that are common to different processes and need not be flushed. The 
    /// CR4.PGE bit controls this feature.
    pge: 13,
    /// Machine Check Architecture. A value of 1 indicates the Machine Check Architecture of 
    /// reporting machine errors is supported. The MCG_CAP MSR contains feature bits describing how 
    /// many banks of error reporting MSRs are supported.
    mca: 14,
    /// Conditional Move Instructions. The conditional move instruction CMOV is supported. In 
    /// addition, if x87 FPU is present as indicated by the CPUID.FPU feature bit, then the FCOMI 
    /// and FCMOV instructions are supported
    cmov: 15,
    /// Page Attribute Table. Page Attribute Table is supported. This feature augments the Memory 
    /// Type Range Registers (MTRRs), allowing an operating system to specify attributes of memory 
    /// accessed through a linear address on a 4KB granularity.
    pat: 16,
    /// 36-Bit Page Size Extension. 4-MByte pages addressing physical memory beyond 4 GBytes are 
    /// supported with 32-bit paging. This feature indicates that upper bits of the physical address
    /// of a 4-MByte page are encoded in bits 20:13 of the page-directory entry. Such physical 
    /// addresses are limited by MAXPHYADDR and may be up to 40 bits in size.
    pse3_36: 17,
    /// Processor Serial Number. The processor supports the 96-bit processor identification number
    /// feature and the feature is enabled.
    psn: 18, 
    /// CLFLUSH Instruction. CLFLUSH Instruction is supported.
    clfsh: 19,
    // Reserved
    /// Debug Store. The processor supports the ability to write debug information into a memory 
    /// resident buffer. This feature is used by the branch trace store (BTS) and processor 
    /// event-based sampling (PEBS) facilities (see Chapter 23, “Introduction to Virtual-Machine 
    /// Extensions,” in the Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 
    /// 3C).
    ds: 21,
    /// Thermal Monitor and Software Controlled Clock Facilities. The processor implements internal 
    /// MSRs that allow processor temperature to be monitored and processor performance to be 
    /// modulated in predefined duty cycles under software control.
    acpi: 22,
    /// Intel MMX Technology. The processor supports the Intel MMX technology.
    mmx: 23,
    /// FXSAVE and FXRSTOR Instructions. The FXSAVE and FXRSTOR instructions are supported for fast
    /// save and restore of the floating point context. Presence of this bit also indicates that 
    /// CR4.OSFXSR is available for an operating system to indicate that it supports the FXSAVE and
    /// FXRSTOR instructions.
    fxsr: 24, 
    /// SSE. The processor supports the SSE extensions.
    sse: 25, 
    /// SSE2. The processor supports the SSE2 extensions.
    sse2: 26, 
    /// Self Snoop. The processor supports the management of conflicting memory types by performing 
    /// a snoop of its own cache structure for transactions issued to the bus.
    ss: 27,
    /// Max APIC IDs reserved field is Valid. A value of 0 for HTT indicates there is only a single 
    /// logical processor in the package and software should assume only a single APIC ID is 
    /// reserved. A value of 1 for HTT indicates the value in CPUID.1.EBX[23:16] (the Maximum number 
    /// of addressable IDs for logical processors in this package) is valid for the package.
    htt: 28, 
    /// Thermal Monitor. The processor implements the thermal monitor automatic thermal control circuitry (TCC).
    tm: 29,
    // Reserved
    /// Pending Break Enable. The processor supports the use of the FERR#/PBE# pin when the 
    /// processor is in the stop-clock state (STPCLK# is asserted) to signal the processor that an 
    /// interrupt is pending and that the processor should return to normal operation to handle the 
    /// interrupt.
    pbe: 31,
});

// -------------------------------------------------------------------------------------------------
// Leaf 4
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf4Eax, u32, {
    /// Cache Type Field.
    /// - 0 = Null - No more caches.
    /// - 1 = Data Cache.
    /// - 2 = Instruction Cache.
    /// - 3 = Unified Cache.
    /// - 4-31 = Reserved.
    cache_type_field: 0..5,
    /// Cache Level (starts at 1).
    cache_level: 5..8,
    /// Self Initializing cache level (does not need SW initialization).
    sicl: 8,
    /// Fully Associative cache.
    fac: 9,
    // Reserved 10..14
    /// Maximum number of addressable IDs for logical processors sharing this cache.
    /// - Add one to the return value to get the result.
    /// - The nearest power-of-2 integer that is not smaller than (1 + EAX[25:14]) is the number of 
    ///   unique initial APIC IDs reserved for addressing different logical processors sharing this 
    ///   cache.
    max_num_addressable_ids_for_logical_processors_sharing_this_cache: 14..26,
    /// Maximum number of addressable IDs for processor cores in the physical package.
    /// - Add one to the return value to get the result.
    /// - The nearest power-of-2 integer that is not smaller than (1 + EAX[31:26]) is the number of
    ///   unique Core_IDs reserved for addressing different processor cores in a physical package. 
    ///   Core ID is a subset of bits of the initial APIC ID.
    /// - The returned value is constant for valid initial values in ECX. Valid ECX values start 
    ///   from 0.
    max_num_addressable_ids_for_processor_cores_in_physical_package: 26..32,
});
#[rustfmt::skip]
bitfield!(Leaf4Ebx, u32, {
    /// L = System Coherency Line Size.
    /// 
    /// Add one to the return value to get the result.
    system_coherency_line_size: 0..12,
    /// P = Physical Line partitions.
    /// 
    /// Add one to the return value to get the result.
    physical_line_partitions: 12..22,
    /// W = Ways of associativity.
    ///
    /// Add one to the return value to get the result.
    ways_of_associativity: 22..32
});
#[rustfmt::skip]
bitfield!(Leaf4Ecx, u32, {
    /// S = Number of Sets.
    ///
    /// Add one to the return value to get the result.
    number_of_sets: 0..32,
});
#[rustfmt::skip]
bitfield!(Leaf4Edx, u32, {
    /// Write-Back Invalidate/Invalidate.
    /// - 0 = WBINVD/INVD from threads sharing this cache acts upon lower level caches for threads 
    ///   sharing this cache.
    /// - 1 = WBINVD/INVD is not guaranteed to act upon lower level caches of non-originating 
    ///   threads sharing this cache.
    write_back_invalidate: 0,
    /// Cache Inclusiveness.
    /// - 0 = Cache is not inclusive of lower cache levels.
    /// - 1 = Cache is inclusive of lower cache levels.
    cache_inclusiveness: 1,
    /// Complex Cache Indexing.
    /// - 0 = Direct mapped cache.
    /// - 1 = A complex function is used to index the cache, potentially using all address bits.
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
    /// Number of C0* sub C-states supported using MWAIT.
    ///
    /// The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, 
    /// not ACPI Cstates.
    c0_states: 0..4,
    /// Number of C1* sub C-states supported using MWAIT.
    ///
    /// The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, 
    /// not ACPI Cstates.
    c1_states: 4..8,
    /// Number of C2* sub C-states supported using MWAIT.
    ///
    /// The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, 
    /// not ACPI Cstates.
    c2_states: 8..12,
    /// Number of C3* sub C-states supported using MWAIT.
    ///
    /// The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, 
    /// not ACPI Cstates.
    c3_states: 12..16,
    /// Number of C4* sub C-states supported using MWAIT.
    ///
    /// The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, 
    /// not ACPI Cstates.
    c4_states: 16..20,
    /// Number of C5* sub C-states supported using MWAIT.
    ///
    /// The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, 
    /// not ACPI Cstates.
    c5_states: 20..24,
    /// Number of C6* sub C-states supported using MWAIT.
    ///
    /// The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, 
    /// not ACPI Cstates.
    c6_states: 24..28,
    /// Number of C7* sub C-states supported using MWAIT.
    ///
    /// The definition of C0 through C7 states for MWAIT extension are processor-specific C-states, 
    /// not ACPI Cstates.
    c7_states: 28..32
});
// -------------------------------------------------------------------------------------------------
// Leaf 6
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf6Eax, u32, {
    /// Digital temperature sensor is supported if set.
    digital_temperature_sensor: 0,
    /// Intel Turbo Boost Technology available (see description of IA32_MISC_ENABLE[38]).
    intel_turbo_boost_technology: 1,
    /// ARAT. APIC-Timer-always-running feature is supported if set.
    arat: 2,
    // Reserved
    /// PLN. Power limit notification controls are supported if set.
    pln: 4,
    /// ECMD. Clock modulation duty cycle extension is supported if set.
    ecmd: 5,
    /// PTM. Package thermal management is supported if set.
    ptm: 6,
    /// HWP. HWP base registers (IA32_PM_ENABLE[bit 0], IA32_HWP_CAPABILITIES, IA32_HWP_REQUEST, 
    /// IA32_HWP_STATUS) are supported if set.
    hwp: 7,
    /// HWP_Notification. IA32_HWP_INTERRUPT MSR is supported if set.
    hwp_notification: 8,
    /// HWP_Activity_Window. IA32_HWP_REQUEST[bits 41:32] is supported if set.
    hwp_activity_window: 9,
    /// HWP_Energy_Performance_Preference. IA32_HWP_REQUEST[bits 31:24] is supported if set.
    hwp_energy_performance: 10,
    /// HWP_Package_Level_Request. IA32_HWP_REQUEST_PKG MSR is supported if set.
    hwp_package_level_request: 11,
    // Reserved
    /// HDC. HDC base registers IA32_PKG_HDC_CTL, IA32_PM_CTL1, IA32_THREAD_STALL MSRs are supported
    /// if set.
    hdc: 13,
    /// Intel® Turbo Boost Max Technology 3.0 available.
    intel_turbo_boost_max_technology_3: 14,
    /// HWP Capabilities. Highest Performance change is supported if set.
    hwp_capabilities: 15,
    /// HWP PECI override is supported if set.
    hwp_peci_override: 16,
    /// Flexible HWP is supported if set.
    flexible_hwp: 17,
    // Fast access mode for the IA32_HWP_REQUEST MSR is supported if set.
    fast_access_mode_for_i32_hwp_request_msr: 18,
    /// HW_FEEDBACK. IA32_HW_FEEDBACK_PTR MSR, IA32_HW_FEEDBACK_CONFIG MSR, 
    /// IA32_PACKAGE_THERM_STATUS MSR bit 26, and IA32_PACKAGE_THERM_INTERRUPT MSR bit 25 are 
    /// supported if set.
    hw_feedback: 19,
    // Ignoring Idle Logical Processor HWP request is supported if set.
    iilp_hwp_r; 20,
    // Reserved 21..=22
    /// Intel® Thread Director supported if set. IA32_HW_FEEDBACK_CHAR and 
    /// IA32_HW_FEEDBACK_THREAD_CONFIG MSRs are supported if set.
    intel_thread_director: 23,
    // Reserved 24..=31
    
});
#[rustfmt::skip]
bitfield!(Leaf6Ebx, u32, {
    /// Number of Interrupt Thresholds in Digital Thermal Sensor.
    number_of_interrupt_thresholds_in_digital_thermal_sensor: 0..4,
    // Reserved 4..=31
});
#[rustfmt::skip]
bitfield!(Leaf6Ecx, u32, {
    /// Hardware Coordination Feedback Capability (Presence of IA32_MPERF and IA32_APERF). The 
    /// capability to provide a measure of delivered processor performance (since last reset of the 
    /// counters), as a percentage of the expected processor performance when running at the TSC
    /// frequency.
    hardware_coordination_feedback_capability: 0,
    // Reserved 1..=2
    /// The processor supports performance-energy bias preference if CPUID.06H:ECX.SETBH[bit 3] is 
    /// set and it also implies the presence of a new architectural MSR called IA32_ENERGY_PERF_BIAS
    /// (1B0H).
    performance_energy_bias: 3,
    /// Reserved 04..=07
    /// Number of Intel® Thread Director classes supported by the processor. Information for that
    /// many classes is written into the Intel Thread Director Table by the hardware.
    intel_thread_director_classes: 8..16,
    // Reserved 16..=31
});
#[rustfmt::skip]
bitfield!(Leaf6Edx, u32, {
    /// Bitmap of supported hardware feedback interface capabilities.
    /// - 0 = When set to 1, indicates support for performance capability reporting.
    /// - 1 = When set to 1, indicates support for energy efficiency capability reporting.
    /// - 2-7 = Reserved
    bitmap_hardware_feedback_interface_capabilities: 0..8,
    /// Enumerates the size of the hardware feedback interface structure in number of 4 KB pages; 
    /// add one to the return value to get the result.
    enum_hardware_feedback_interface_4k: 8..12,
    /// Index (starting at 0) of this logical processor's row in the hardware feedback interface
    /// structure. Note that on some parts the index may be same for multiple logical processors. On
    /// some parts the indices may not be contiguous, i.e., there may be unused rows in the hardware
    /// feedback interface structure.
    index: 16..32
});
// -------------------------------------------------------------------------------------------------
// Leaf 7
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(Leaf7Subleaf0Eax, u32, {
    /// Reports the maximum input value for supported leaf 7 sub-leaves.
    max_input_value_subleaf: 0..32
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf0Ebx, u32, {
    /// FSGSBASE. Supports RDFSBASE/RDGSBASE/WRFSBASE/WRGSBASE if 1.
    fsgsbase: 0,
    /// IA32_TSC_ADJUST MSR is supported if 1.
    ia32_tsc_adjust_msr: 1,
    /// SGX. Supports Intel® Software Guard Extensions (Intel® SGX Extensions) if 1.
    sgx: 2,
    /// BMI1.
    bmi1: 3,
    /// HLE.
    hle: 4,
    /// AVX2.
    avx2: 5,
    /// FDP_EXCPTN_ONLY. x87 FPU Data Pointer updated only on x87 exceptions if 1.
    fdp_excptn_only: 6,
    /// SMEP. Supports Supervisor-Mode Execution Prevention if 1.
    smep: 7,
    /// BMI2.
    bmi2: 8,
    /// Supports Enhanced REP MOVSB/STOSB if 1.
    suports_enhanced_rep_movsb_stosb: 9,
    /// INVPCID. If 1, supports INVPCID instruction for system software that manages process-context 
    /// identifiers.
    invpcid: 10,
    /// RTM.
    rtm: 11,
    /// RDT-M. Supports Intel® Resource Director Technology (Intel® RDT) Monitoring capability if 1.
    rdt_m: 12,
    /// Deprecates FPU CS and FPU DS values if 1.
    deprecates_fpu_cs_and_fpu_ds: 13,
    /// MPX. Supports Intel® Memory Protection Extensions if 1.
    mpx: 14,
    /// RDT-A. Supports Intel® Resource Director Technology (Intel® RDT) Allocation capability if 1.
    rdt_t: 15,
    /// AVX512F.
    avx512f: 16,
    /// AVX512DQ.
    avx512dq: 17,
    /// RDSEED.
    rdseed: 18,
    /// ADX.
    adx: 19,
    /// SMAP. Supports Supervisor-Mode Access Prevention (and the CLAC/STAC instructions) if 1.
    smap: 20,
    /// AVX512_IFMA.
    avx512_ifma: 21,
    // Reserved
    /// CLFLUSHOPT.
    clfushopt: 23,
    /// CLWB.
    clwb: 24,
    /// Intel Processor Trace.
    intel_processor_trace: 25,
    /// AVX512PF. (Intel® Xeon Phi™ only.)
    avx512pf: 26,
    /// AVX512ER. (Intel® Xeon Phi™ only.)
    avx512er: 27,
    /// AVX512CD.
    avx512cd: 28,
    /// SHA. supports Intel® Secure Hash Algorithm Extensions (Intel® SHA Extensions) if 1.
    sha: 29,
    /// AVX512BW.
    avx512bw: 30,
    /// AVX512VL.
    avx512vl: 31
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf0Ecx, u32, {
    /// PREFETCHWT1. (Intel® Xeon Phi™ only.)
    prefetchwt1: 0,
    /// AVX512_VBMI.
    avx512_vbmi: 1,
    /// UMIP. Supports user-mode instruction prevention if 1.
    umip: 2,
    /// PKU. Supports protection keys for user-mode pages if 1.
    pku: 3,
    /// OSPKE. If 1, OS has set CR4.PKE to enable protection keys (and the RDPKRU/WRPKRU instructions).
    ospke: 4,
    /// WAITPKG.
    waitpkg: 5,
    /// AVX512_VBMI2.
    avx512_vbmi2: 6,
    /// CET_SS. Supports CET shadow stack features if 1. Processors that set this bit define bits 
    /// 1:0 of the IA32_U_CET and IA32_S_CET MSRs. Enumerates support for the following MSRs: 
    /// IA32_INTERRUPT_SPP_TABLE_ADDR, IA32_PL3_SSP, IA32_PL2_SSP, IA32_PL1_SSP, and IA32_PL0_SSP.
    cet_ss: 7,
    /// GFNI.
    gfni: 8,
    /// VAES.
    vaes: 9,
    /// VPCLMULQDQ.
    vpclmulqdq: 10,
    /// AVX512_VNNI.
    avx512_vnni: 11,
    /// AVX512_BITALG.
    avx512_bitalg: 12,
    /// TME_EN. If 1, the following MSRs are supported: IA32_TME_CAPABILITY, IA32_TME_ACTIVATE, 
    /// IA32_TME_EXCLUDE_MASK, and IA32_TME_EXCLUDE_BASE.
    tme_en; 13,
    /// AVX512_VPOPCNTDQ.
    avx512_vpopcntdq: 14,
    // Reserved
    /// LA57. Supports 57-bit linear addresses and five-level paging if 1.
    la57: 16,
    /// The value of MAWAU used by the BNDLDX and BNDSTX instructions in 64-bit mode.
    value_of_mawau: 17..22,
    /// RDPID and IA32_TSC_AUX are available if 1.
    rdpid_and_ia32_tsc_aux: 22,
    /// KL. Supports Key Locker if 1.
    kl: 23,
    // Reserved
    /// CLDEMOTE. Supports cache line demote if 1.
    cldemote: 25,
    // Reserved
    /// MOVDIRI. Supports MOVDIRI if 1.
    movdiri: 27,
    /// MOVDIR64B. Supports MOVDIR64B if 1.
    movdiri64b: 28,
    // Reserved
    /// SGX_LC. Supports SGX Launch Configuration if 1.
    sgx_lc: 30,
    /// PKS. Supports protection keys for supervisor-mode pages if 1.
    pks: 31
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf0Edx, u32, {
    // Reserved
    /// AVX512_4VNNIW. (Intel® Xeon Phi™ only.)
    avx512_4vnniw: 2,
    /// AVX512_4FMAPS. (Intel® Xeon Phi™ only.)
    avx512_4fmaps: 3,
    /// Fast Short REP MOV.
    fast_short_rep_mov: 4,
    // Reserved 5..=7
    /// AVX512_VP2INTERSECT.
    avx512_vp2intersect: 8,
    // Reserved
    /// MD_CLEAR supported.
    md_clear: 10,
    // Reserved
    /// SERIALIZE.
    serialize:  11..14,
    /// Hybrid. If 1, the processor is identified as a hybrid part.
    hydrid: 15,
    // Reserved 16..=17
    /// PCONFIG. Supports PCONFIG if 1.
    pconfig: 18,
    // Reserved
    /// CET_IBT. Supports CET indirect branch tracking features if 1. Processors that set this bit 
    /// define bits 5:2 and bits 63:10 of the IA32_U_CET and IA32_S_CET MSRs.
    cet_ibt: 19,
    // Reserved 21..=25
    /// Enumerates support for indirect branch restricted speculation (IBRS) and the indirect branch
    /// predictorn barrier (IBPB). Processors that set this bit support the IA32_SPEC_CTRL MSR and 
    /// the A32_PRED_CMD MSR. They allow software to set IA32_SPEC_CTRL[0] (IBRS) and 
    /// IA32_PRED_CMD[0] (IBPB).
    ibrs_ibpb_enum: 26,
    /// Enumerates support for single thread indirect branch predictors (STIBP). Processors that set
    /// this bit support the IA32_SPEC_CTRL MSR. They allow software to set IA32_SPEC_CTRL[1] 
    /// (STIBP).
    stibp_enum: 27,
    /// Enumerates support for L1D_FLUSH. Processors that set this bit support the IA32_FLUSH_CMD 
    /// MSR. They allow software to set IA32_FLUSH_CMD[0] (L1D_FLUSH).
    l1d_flush_enum: 28,
    /// Enumerates support for the IA32_ARCH_CAPABILITIES MSR.
    ia32_arch_capabilities_msr_enum: 29,
    /// Enumerates support for the IA32_CORE_CAPABILITIES MSR.
    ia32_core_capabilities_msr_enum: 30,
    /// Enumerates support for Speculative Store Bypass Disable (SSBD). Processors that set this bit
    /// support the IA32_SPEC_CTRL MSR. They allow software to set IA32_SPEC_CTRL[2] (SSBD).
    ssbd_enum: 31,
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf1Eax, u32, {
    // Reserved 0..=3
    /// AVX-VNNI. AVX (VEX-encoded) versions of the Vector Neural Network Instructions.
    avx_vnni: 4,
    /// AVX512_BF16. Vector Neural Network Instructions supporting BFLOAT16 inputs and conversion 
    /// instructions from IEEE single precision.
    avx512_bf16: 5,
    // Reserved 6..=9
    /// If 1, supports fast zero-length REP MOVSB.
    fast_zero_length_rep_movsh: 10,
    /// If 1, supports fast short REP STOSB.
    fast_short_rep_stosb: 11,
    /// If 1, supports fast short REP CMPSB, REP SCASB.
    fast_short_rep_cmpsb_rep_scasb: 12,
    // Reserved 13..=21
    /// HRESET. If 1, supports history reset via the HRESET instruction and the IA32_HRESET_ENABLE 
    /// MSR. When set, indicates that the Processor History Reset Leaf (EAX = 20H) is valid.
    hreset: 22,
    // Reserved 23..=31
});
#[rustfmt::skip]
bitfield!(Leaf7Subleaf1Ebx, u32, {
    /// Enumerates the presence of the IA32_PPIN and IA32_PPIN_CTL MSRs. If 1, these MSRs are
    /// supported.
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
    /// Value of bits [31:0] of IA32_PLATFORM_DCA_CAP MSR (address 1F8H).
    ia32_platform_dca_cap_msr: 0..32
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
    /// Version ID of architectural performance monitoring.
    version_id_of_architectural_performance_monitoring: 0..8,
    /// Number of general-purpose performance monitoring counter per logical processor.
    num_perf_monitor_counter_per_logical_processor: 8..16,
    /// Bit width of general-purpose, performance monitoring counter.
    bot_width_perf_monitor_counter: 16..24,
    /// Length of EBX bit vector to enumerate architectural performance monitoring events. 
    /// Architectural event x is supported if EBX[x]=0 && EAX[31:24]>x.
    len_ebx_bit_vec: 24..32
});
#[rustfmt::skip]
bitfield!(LeafAEbx, u32, {
    /// Core cycle event not available if 1 or if EAX[31:24]<1.
    core_cycle_event: 0,
    /// Instruction retired event not available if 1 or if EAX[31:24]<2.
    instruction_retired_event: 1,
    /// Reference cycles event not available if 1 or if EAX[31:24]<3.
    reference_cycles_event: 2,
    /// Last-level cache reference event not available if 1 or if EAX[31:24]<4.
    last_level_cache_reference_event: 3,
    /// Last-level cache misses event not available if 1 or if EAX[31:24]<5.
    last_level_cache_misses_event: 4,
    /// Branch instruction retired event not available if 1 or if EAX[31:24]<6.
    branch_instruction_retired_event: 5,
    /// Branch mispredict retired event not available if 1 or if EAX[31:24]<7.
    branch_mispredict_retired_event: 6,
    /// Top-down slots event not available if 1 or if EAX[31:24]<8.
    top_down_slots_event: 7,
    // Reserved 8..=31
});
#[rustfmt::skip]
bitfield!(LeafAEcx, u32, {
    /// Supported fixed counters bit mask. Fixed-function performance counter 'i' is supported if 
    /// bit ‘i’ is 1 (first counter index starts at zero). It is recommended to use the following 
    /// logic to determine if a Fixed Counter is supported: 
    /// FxCtr[i]_is_supported := ECX[i] || (EDX[4:0] > i);
    supported_fixed_counters_bit_mask: 0..32
});
#[rustfmt::skip]
bitfield!(LeafAEdx, u32, {
    /// Number of contiguous fixed-function performance counters starting from 0 (if Version ID >1).
    contigous_fixed_function_performance_counter: 0..5,
    /// Bit width of fixed-function performance counters (if Version ID > 1).
    bit_width_of_fixed_function_performnace_counter: 5..13,
    // Reserved 13..=14
    /// AnyThread deprecation.
    anythread_deprecation: 15
    // Reserved 16..=31
});
// -------------------------------------------------------------------------------------------------
// Leaf B
// -------------------------------------------------------------------------------------------------
#[rustfmt::skip]
bitfield!(LeafBEax, u32, {
    /// Number of bits to shift right on x2APIC ID to get a unique topology ID of the next level 
    /// type*. All logical processors with the same next level ID share current level.
    ///
    /// *Software should use this field (EAX[4:0]) to enumerate processor topology of the system.
    bit_shifts_right_2x_apic_id_unique_topology_id: 0..5
});
#[rustfmt::skip]
bitfield!(LeafBEbx, u32, {
    /// Number of logical processors at this level type. The number reflects configuration as shipped
    /// by Intel**.
    ///
    /// **Software must not use EBX[15:0] to enumerate processor topology of the system. This value 
    /// in this field (EBX[15:0]) is only intended for display/diagnostic purposes. The actual 
    /// number of  logical processors available to BIOS/OS/Applications may be different from the 
    /// value of  EBX[15:0], depending on software and platform hardware configurations.
    logical_processors: 0..16
});
#[rustfmt::skip]
bitfield!(LeafBEcx, u32, {
    /// Level number. Same value in ECX input.
    level_number: 0..8,
    /// Level type***
    /// 
    /// If an input value n in ECX returns the invalid level-type of 0 in ECX[15:8], other input 
    /// values with ECX>n also return 0 in ECX[15:8].
    ///
    /// ***The value of the “level type” field is not related to level numbers in any way, higher 
    /// “level type” values do not mean higher levels. Level type field has the following encoding:
    /// - 0: Invalid.
    /// - 1: SMT.
    /// - 2: Core.
    /// - 3-255: Reserved.
    level_type: 8..16
    // Reserved 16..=31
});
#[rustfmt::skip]
bitfield!(LeafBEdx, u32, {
    /// x2APIC ID the current logical processor.
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
    /// x87 state.
    x86_state: 0,
    /// SSE state.
    sse_state: 1,
    /// AVX state.
    avx_state: 2,
    /// MPX state.
    mpx_state: 3..5,
    /// AVX-512 state.
    avx512_state: 5..8,
    /// Used for IA32_XSS.
    used_for_ia32_xss: 8,
    /// PKRU state.
    pkru_state: 9,
    // Reserved 10..=12
    /// Used for IA32_XSS.
    used_for_ia32_xss_1: 13,
    // Reserved 14..=15
    /// Used for IA32_XSS.
    used_for_ia32_xss_2: 16,
    // Reserved 17..=31
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf0Ebx, u32, {
    /// Maximum size (bytes, from the beginning of the XSAVE/XRSTOR save area) required by enabled 
    /// features in XCR0. May be different than ECX if some features at the end of the XSAVE save
    /// area are not enabled.
    maximum_size: 0..32
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf0Ecx, u32, {
    /// Maximum size (bytes, from the beginning of the XSAVE/XRSTOR save area) of the XSAVE/XRSTOR 
    /// save area required by all supported features in the processor, i.e., all the valid bit 
    /// fields in XCR0.
    ///
    // `LeafDSubleaf0Ecx::maximum_size() >= LeafDSubleaf0Ebx::maximum_size()`
    maximum_size: 0..32
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf0Edx, u32, {
    // TODO Double check this
    // Reports the supported bits of the upper 32 bits of XCR0. XCR0[n+32] can be set to 1 only if 
    // EDX[n] is 1.
    // Reserved
});
// Leaf 1
#[rustfmt::skip]
bitfield!(LeafDSubleaf1Eax, u32, {
    /// XSAVEOPT is available.
    xsaveopt_available: 0,
    /// Supports XSAVEC and the compacted form of XRSTOR if set.
    xsavec_compacted_xrstor: 1,
    /// Supports XGETBV with ECX = 1 if set.
    xgetbv: 2,
    /// Supports XSAVES/XRSTORS and IA32_XSS if set.
    xsaves_xrstors_ia32_xss: 3,
    // Reserved 0..32
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf1Ebx, u32, {
    /// The size in bytes of the XSAVE area containing all states enabled by XCRO | IA32_XSS.
    xsave_size: 0..32,
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf1Ecx, u32, {
    // Reports the supported bits of the lower 32 bits of the IA32_XSS MSR. IA32_XSS[n] can be set 
    // to 1 only if ECX[n] is 1.
    /// Used for XCR0.
    xcr0_1: 0..8,
    /// PT state.
    pt_state: 8,
    /// Used for XCR0.
    xcr0_2: 9,
    // Reserved
    /// CET user state.
    cet_user_state: 11,
    /// CET supervisor state.
    cet_supervisor_state: 12,
    /// HDC state.
    hdc_state: 13,
    // Reserved
    /// LBR state (architectural).
    lbr_state: 15,
    /// HWP state.
    hwp_state: 16,
    // Reserved 17..=31
});
#[rustfmt::skip]
bitfield!(LeafDSubleaf1Edx, u32, {
    // Reports the supported bits of the upper 32 bits of the IA32_XSS MSR. IA32_XSS[n+32] can be 
    // set to 1 only if EDX[n] is 1.
    // Reserved
});
// Leaf >1
#[rustfmt::skip]
bitfield!(LeafDSubleafGt1Eax, u32, {
    /// The size in bytes (from the offset specified in EBX) of the save area for an extended state 
    /// feature associated with a valid sub-leaf index, n.
    save_area_size: 0..32,
});
#[rustfmt::skip]
bitfield!(LeafDSubleafGt1Ebx, u32, {
    /// The offset in bytes of this extended state component’s save area from the beginning of the 
    /// XSAVE/XRSTOR area.
    /// 
    /// This field reports 0 if the sub-leaf index, n, does not map to a valid bit in the XCR0 
    /// register*.
    ///
    /// *If ECX contains an invalid sub-leaf index, EAX/EBX/ECX/EDX return 0. Sub-leaf n 
    /// (0 ≤ n ≤ 31) is invalid if sub-leaf 0 returns 0 in EAX[n] and sub-leaf 1 returns 0 in 
    /// ECX[n]. Sub-leaf n (32 ≤ n ≤ 63) is invalid if sub-leaf 0 returns 0 in EDX[n-32] and 
    /// sub-leaf 1 returns 0 in EDX[n-32].
    save_area_offset: 0..32
});
#[rustfmt::skip]
bitfield!(LeafDSubleafGt1Ecx, u32, {
    /// Is set if the bit n (corresponding to the sub-leaf index) is supported in the IA32_XSS MSR; 
    /// it is clear if bit n is instead supported in XCR0.
    ///
    /// This field reports 0 if the sub-leaf index, n, is invalid*.
    ///
    /// *If ECX contains an invalid sub-leaf index, EAX/EBX/ECX/EDX return 0. Sub-leaf n 
    /// (0 ≤ n ≤ 31) is invalid if sub-leaf 0 returns 0 in EAX[n] and sub-leaf 1 returns 0 in 
    /// ECX[n]. Sub-leaf n (32 ≤ n ≤ 63) is invalid if sub-leaf 0 returns 0 in EDX[n-32] and 
    /// sub-leaf 1 returns 0 in EDX[n-32].
    supported_ia32_xss_msr: 0,
    /// Is set if, when the compacted format of an XSAVE area is used, this extended state component
    /// located on the next 64-byte boundary following the preceding state component (otherwise, it 
    /// is located immediately following the preceding state component).
    ///
    /// This field reports 0 if the sub-leaf index, n, is invalid*.
    ///
    /// *If ECX contains an invalid sub-leaf index, EAX/EBX/ECX/EDX return 0. Sub-leaf n 
    /// (0 ≤ n ≤ 31) is invalid if sub-leaf 0 returns 0 in EAX[n] and sub-leaf 1 returns 0 in 
    /// ECX[n]. Sub-leaf n (32 ≤ n ≤ 63) is invalid if sub-leaf 0 returns 0 in EDX[n-32] and 
    /// sub-leaf 1 returns 0 in EDX[n-32].
    compacted_xsave_used: 1,
    // 0..=31 reserved
});
#[rustfmt::skip]
bitfield!(LeafDSubleafGt1Edx, u32, {
    // This field reports 0 if the sub-leaf index, n, is invalid*; otherwise it is reserved.
    // 0..=31 reserved
});
// -------------------------------------------------------------------------------------------------
// Leaf F
// -------------------------------------------------------------------------------------------------
// Leaf 0
#[rustfmt::skip]
bitfield!(LeafFSubleaf0Eax, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(LeafFSubleaf0Ebx, u32, {
    /// Maximum range (zero-based) of RMID within this physical processor of all types.
    max_rmid_range: 0..32,
});
#[rustfmt::skip]
bitfield!(LeafFSubleaf0Ecx, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(LeafFSubleaf0Edx, u32, {
    // Reserved
    /// Supports L3 Cache Intel RDT Monitoring if 1.
    l3_rdt_monitor: 1,
    // 2..=32 reserved
});
// Leaf 1
#[rustfmt::skip]
bitfield!(LeafFSubleaf1Eax, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(LeafFSubleaf1Ebx, u32, {
    /// Conversion factor from reported IA32_QM_CTR value to occupancy metric (bytes) and Memory 
    /// Bandwidth Monitoring (MBM) metrics.
    ia32_qm_ctr_conv_factor: 0..32,
});
#[rustfmt::skip]
bitfield!(LeafFSubleaf1Ecx, u32, {
    /// Maximum range (zero-based) of RMID of this resource type.
    rmid_max: 0..32,
});
#[rustfmt::skip]
bitfield!(LeafFSubleaf1Edx, u32, {
    /// Supports L3 occupancy monitoring if 1.
    l3_occupancy_monitor: 0,
    /// Supports L3 Total Bandwidth monitoring if 1.
    l3_total_band_monitor: 1,
    /// Supports L3 Local Bandwidth monitoring if 1.
    l3_local_band_monitor: 2,
    // 0..=31 reserved
});
// -------------------------------------------------------------------------------------------------
// Leaf 10
// -------------------------------------------------------------------------------------------------
// Leaf 0
#[rustfmt::skip]
bitfield!(Leaf10Subleaf0Eax, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf0Ebx, u32, {
    // Reserved
    /// Supports L3 Cache Allocation Technology if 1.
    l3_alloc: 1,
    /// Supports L2 Cache Allocation Technology if 1.
    l2_alloc: 2,
    /// Supports Memory Bandwidth Allocation if 1.
    mem_band_alloc: 3,
    // 04..=31 reserved
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf0Ecx, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf0Edx, u32, {
    // Reserved
});
// Leaf 1
#[rustfmt::skip]
bitfield!(Leaf10Subleaf1Eax, u32, {
    /// Length of the capacity bit mask for the corresponding ResID. Add one to the return value to 
    /// get the result.
    len_cap_resid_mask: 0..5,
    // 5..=31 reserved
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf1Ebx, u32, {
    /// Bit-granular map of isolation/contention of allocation units.
    granular_iso_cont_map: 0..32,
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf1Ecx, u32, {
    // 0..=1 reserved
    /// Code and Data Prioritization Technology supported if 1.
    cd_prior: 2,
    // 3..=31 reserved
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf1Edx, u32, {
    /// Highest COS number supported for this ResID.
    highest_cos_resid: 0..16,
    // 0..=31 reserved
});
// Leaf 2
type Leaf10Subleaf2Eax = Leaf10Subleaf1Eax;
type Leaf10Subleaf2Ebx = Leaf10Subleaf1Ebx;
type Leaf10Subleaf2Ecx = Leaf10Subleaf1Ecx;
type Leaf10Subleaf2Edx = Leaf10Subleaf1Edx;
// Leaf 3
#[rustfmt::skip]
bitfield!(Leaf10Subleaf3Eax, u32, {
    /// Reports the maximum MBA throttling value supported for the corresponding ResID. Add one to 
    /// the return value to get the result.
    max_mba_throt_resid: 0..12,
    // reserved 12..=31
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf3Ebx, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf3Ecx, u32, {
    // 0..=1 reserved
    /// Reports whether the response of the delay values is linear.
    linear_response_delay_values: 2,
    // 3..=31 reserved
});
#[rustfmt::skip]
bitfield!(Leaf10Subleaf3Edx, u32, {
    /// Highest COS number supported for this ResID.
    highest_cos_resid: 0..16,
    // 16..=31 reserved
});
// -------------------------------------------------------------------------------------------------
// Leaf 12
// -------------------------------------------------------------------------------------------------
// Leaf 0
#[rustfmt::skip]
bitfield!(Leaf12Subleaf0Eax, u32, {
    /// SGX1. If 1, Indicates Intel SGX supports the collection of SGX1 leaf functions.
    sgx1: 0,
    /// SGX2. If 1, Indicates Intel SGX supports the collection of SGX2 leaf functions.
    sgx2: 1,
    // 2..=4 reserved
    /// If 1, indicates Intel SGX supports ENCLV instruction leaves EINCVIRTCHILD, EDECVIRTCHILD,
    /// and ESETCONTEXT.
    enclv: 5,
    /// If 1, indicates Intel SGX supports ENCLS instruction leaves ETRACKC, ERDINFO, ELDBC, and 
    /// ELDUC.
    encls: 6,
    // 7..=31 reserved
});
#[rustfmt::skip]
bitfield!(Leaf12Subleaf0Ebx, u32, {
    /// MISCSELECT. Bit vector of supported extended SGX features.
    miscselect: 0..32,
});
#[rustfmt::skip]
bitfield!(Leaf12Subleaf0Ecx, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf12Subleaf0Edx, u32, {
    /// MaxEnclaveSize_Not64. The maximum supported enclave size in non-64-bit mode is 2^(EDX[7:0]).
    max_enclave_size_not_64: 0..8,
    /// MaxEnclaveSize_64. The maximum supported enclave size in 64-bit mode is 2^(EDX[15:8]).
    max_enclave_size_64: 8..16,
    // 16..=31 reserved
});
// Leaf 1
#[rustfmt::skip]
bitfield!(Leaf12Subleaf1Eax, u32, {
    /// Reports the valid bits of SECS.ATTRIBUTES[31:0] that software can set with ECREATE.
    ecreate_attrs_0_31: 0..32,
});
#[rustfmt::skip]
bitfield!(Leaf12Subleaf1Ebx, u32, {
    /// Reports the valid bits of SECS.ATTRIBUTES[63:32] that software can set with ECREATE.
    ecreate_attrs_32_63: 0..32,
});
#[rustfmt::skip]
bitfield!(Leaf12Subleaf1Ecx, u32, {
    /// Reports the valid bits of SECS.ATTRIBUTES[95:64] that software can set with ECREATE.
    ecreate_attrs_64_95: 0..32,
});
#[rustfmt::skip]
bitfield!(Leaf12Subleaf1Edx, u32, {
    /// Reports the valid bits of SECS.ATTRIBUTES[127:96] that software can set with ECREATE.
    ecreate_attrs_96_127: 0..32,
});
// Leaf >1
#[rustfmt::skip]
bitfield!(Leaf12SubleafGt1Eax, u32, {
    /// Sub-leaf Type
    /// - 0000b: Indicates this sub-leaf is invalid.
    /// - 0001b: This sub-leaf enumerates an EPC section. EBX:EAX and EDX:ECX provide information on the
    /// Enclave Page Cache (EPC) section.
    /// All other type encodings are reserved.
    subleaf_type: 0..4,
    /// Bits 31:12 of the physical address of the base of the EPC section.
    ///
    /// When EAX[03:00] = 0001b (otherwise 0)
    epc_base_31_12: 12..32,
});
#[rustfmt::skip]
bitfield!(Leaf12SubleafGt1Ebx, u32, {
    /// Bits 51:32 of the physical address of the base of the EPC section.
    ///
    /// When EAX[03:00] = 0001b (otherwise 0)
    epc_base_51_32: 0..20,
    // 20..=31 reserved
});
#[rustfmt::skip]
bitfield!(Leaf12SubleafGt1Ecx, u32, {
    /// EPC section property encoding defined as follows:
    /// - If ECX[3:0] = 0000b, then all bits of the EDX:ECX pair are enumerated as 0.
    /// - If ECX[3:0] = 0001b, then this section has confidentiality and integrity protection.
    /// - If ECX[3:0] = 0010b, then this section has confidentiality protection only.
    /// All other encodings are reserved.
    ///
    /// When EAX[03:00] = 0001b (otherwise 0)
    epc_section: 0..4,
    // 4..=11 reserved
    /// Bits 31:12 of the size of the corresponding EPC section within the Processor Reserved 
    /// Memory.
    ///
    /// When EAX[03:00] = 0001b (otherwise 0)
    epc_reserved_31_12: 12..32,

});
#[rustfmt::skip]
bitfield!(Leaf12SubleafGt1Edx, u32, {
    /// Bits 51:32 of the size of the corresponding EPC section within the Processor Reserved 
    /// Memory.
    ///
    /// When EAX[03:00] = 0001b (otherwise 0)
    epc_reserved_51_32: 0..20,
    // 20..=31 reserved
});
// -------------------------------------------------------------------------------------------------
// Leaf 14
// -------------------------------------------------------------------------------------------------
// Leaf 0
#[rustfmt::skip]
bitfield!(Leaf14Subleaf0Eax, u32, {
    /// Reports the maximum sub-leaf supported in leaf 14H.
    ///
    /// **At the moment of writing the Intel specification only notes the format of ECX=1, therefore
    /// this field should only be 0 or 1**
    max_subleaf: 0..32,
});
#[rustfmt::skip]
bitfield!(Leaf14Subleaf0Ebx, u32, {
    /// If 1, indicates that IA32_RTIT_CTL.CR3Filter can be set to 1, and that IA32_RTIT_CR3_MATCH
    /// MSR can be accessed.
    ia32_rtit: 0,
    /// If 1, indicates support of Configurable PSB and Cycle-Accurate Mode.
    psb_config_cam: 1,
    /// If 1, indicates support of IP Filtering, TraceStop filtering, and preservation of Intel PT 
    /// MSRs across warm reset.
    ip_filtering_and_ts_filtering_and_pt_msr_preservation: 2,
    /// If 1, indicates support of MTC timing packet and suppression of COFI-based packets.
    mtc_timing_and_cofi_suppression: 3,
    /// If 1, indicates support of PTWRITE. Writes can set IA32_RTIT_CTL[12] (PTWEn) and 
    /// IA32_RTIT_CTL[5] (FUPonPTW), and PTWRITE can generate packets.
    ptwrite: 4,
    /// If 1, indicates support of Power Event Trace. Writes can set IA32_RTIT_CTL[4] (PwrEvtEn), 
    /// enabling Power Event Trace packet generation.
    power_event_trace: 5,
    /// If 1, indicates support for PSB and PMI preservation. Writes can set IA32_RTIT_CTL[56] 
    /// (InjectPsbPmiOnEnable), enabling the processor to set IA32_RTIT_STATUS[7] (PendTopaPMI) 
    /// and/or IA32_RTIT_STATUS[6] (PendPSB) in order to preserve ToPA PMIs and/or PSBs otherwise 
    /// lost due to Intel PT disable. Writes can also set PendToPAPMI and PendPSB.
    psb_and_pmi_preservation: 6,
    /// If 1, writes can set IA32_RTIT_CTL[31] (EventEn), enabling Event Trace packet generation.
    ia32_rtit_ctl_31: 7,
    /// If 1, writes can set IA32_RTIT_CTL[55] (DisTNT), disabling TNT packet generation.
    ia32_rtit_ctl_55: 8,
    // 9..=31 reserved
});
#[rustfmt::skip]
bitfield!(Leaf14Subleaf0Ecx, u32, {
    /// If 1, Tracing can be enabled with IA32_RTIT_CTL.ToPA = 1, hence utilizing the ToPA output 
    /// scheme; IA32_RTIT_OUTPUT_BASE and IA32_RTIT_OUTPUT_MASK_PTRS MSRs can be accessed.
    ia32_rtit_ctl_topa: 0,
    /// If 1, ToPA tables can hold any number of output entries, up to the maximum allowed by the 
    /// MaskOrTableOffset field of IA32_RTIT_OUTPUT_MASK_PTRS.
    topa_ext: 1,
    /// If 1, indicates support of Single-Range Output scheme.
    sros: 2,
    /// If 1, indicates support of output to Trace Transport subsystem.
    otts: 3,
    // 4..=30 reserved
    /// If 1, generated packets which contain IP payloads have LIP values, which include the CS base component.
    lip_cs_base: 31,
});
#[rustfmt::skip]
bitfield!(Leaf14Subleaf0Edx, u32, {
    /// Reserved
});
// Leaf 1
#[rustfmt::skip]
bitfield!(Leaf14Subleaf1Eax, u32, {
    /// Number of configurable Address Ranges for filtering.
    configurable_filterig_addr_ranges: 0..3,
    // 3..=15 reserved
    /// Bitmap of supported MTC period encodings.
    mtc_period_encodings: 16..32,
});
#[rustfmt::skip]
bitfield!(Leaf14Subleaf1Ebx, u32, {
    /// Bitmap of supported Cycle Threshold value encodings.
    cycle_threshold_value_encodings: 0..16,
    /// Bitmap of supported Configurable PSB frequency encodings.
    configurable_psb_freq_encodings: 16..32,
});
#[rustfmt::skip]
bitfield!(Leaf14Subleaf1Ecx, u32, {
    // Reserved
});
#[rustfmt::skip]
bitfield!(Leaf14Subleaf1Edx, u32, {
    // Reserved
});
// -------------------------------------------------------------------------------------------------
// Leaf types
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
type LeafD = (LeafDSubleaf0, LeafDSubleaf1, Vec<LeafDSubleafGt1>);
type LeafDSubleaf0 = Leaf<LeafDSubleaf0Eax, LeafDSubleaf0Ebx, LeafDSubleaf0Ecx, LeafDSubleaf0Edx>;
type LeafDSubleaf1 = Leaf<LeafDSubleaf1Eax, LeafDSubleaf1Ebx, LeafDSubleaf1Ecx, LeafDSubleaf1Edx>;
type LeafDSubleafGt1 =
    Leaf<LeafDSubleafGt1Eax, LeafDSubleafGt1Ebx, LeafDSubleafGt1Ecx, LeafDSubleafGt1Edx>;
type LeafF = (LeafFSubleaf0, Option<LeafFSubleaf1>);
type LeafFSubleaf0 = Leaf<LeafFSubleaf0Eax, LeafFSubleaf0Ebx, LeafFSubleaf0Ecx, LeafFSubleaf0Edx>;
type LeafFSubleaf1 = Leaf<LeafFSubleaf1Eax, LeafFSubleaf1Ebx, LeafFSubleaf1Ecx, LeafFSubleaf1Edx>;
type Leaf10 = (
    Leaf10Subleaf0,
    Option<Leaf10Subleaf1>,
    Option<Leaf10Subleaf2>,
    Option<Leaf10Subleaf3>,
);
type Leaf10Subleaf0 =
    Leaf<Leaf10Subleaf0Eax, Leaf10Subleaf0Ebx, Leaf10Subleaf0Ecx, Leaf10Subleaf0Edx>;
type Leaf10Subleaf1 =
    Leaf<Leaf10Subleaf1Eax, Leaf10Subleaf1Ebx, Leaf10Subleaf1Ecx, Leaf10Subleaf1Edx>;
type Leaf10Subleaf2 =
    Leaf<Leaf10Subleaf2Eax, Leaf10Subleaf2Ebx, Leaf10Subleaf2Ecx, Leaf10Subleaf2Edx>;
type Leaf10Subleaf3 =
    Leaf<Leaf10Subleaf3Eax, Leaf10Subleaf3Ebx, Leaf10Subleaf3Ecx, Leaf10Subleaf3Edx>;
type Leaf12 = (
    Leaf12Subleaf0,
    Option<Leaf12Subleaf1>,
    Vec<Leaf12SubleafGt1>,
);
type Leaf12Subleaf0 =
    Leaf<Leaf12Subleaf0Eax, Leaf12Subleaf0Ebx, Leaf12Subleaf0Ecx, Leaf12Subleaf0Edx>;
type Leaf12Subleaf1 =
    Leaf<Leaf12Subleaf1Eax, Leaf12Subleaf1Ebx, Leaf12Subleaf1Ecx, Leaf12Subleaf1Edx>;
type Leaf12SubleafGt1 =
    Leaf<Leaf12SubleafGt1Eax, Leaf12SubleafGt1Ebx, Leaf12SubleafGt1Ecx, Leaf12SubleafGt1Edx>;
type Leaf14 = (Leaf14Subleaf0, Option<Leaf14Subleaf1>);
type Leaf14Subleaf0 =
    Leaf<Leaf14Subleaf0Eax, Leaf14Subleaf0Ebx, Leaf14Subleaf0Ecx, Leaf14Subleaf0Edx>;
type Leaf14Subleaf1 =
    Leaf<Leaf14Subleaf1Eax, Leaf14Subleaf1Ebx, Leaf14Subleaf1Ecx, Leaf14Subleaf1Edx>;
// -------------------------------------------------------------------------------------------------
// Supports
// -------------------------------------------------------------------------------------------------
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
        self.eax.smallest_monitor_line_size <= other.eax.smallest_monitor_line_size
            && self.ebx.largest_monitor_line_size >= other.ebx.largest_monitor_line_size
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
                .number_of_interrupt_thresholds_in_digital_thermal_sensor
                >= other
                    .ebx
                    .number_of_interrupt_thresholds_in_digital_thermal_sensor
        // TODO ecx
        // TODO edx
    }
}
impl Leaf7Subleaf0 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        self.eax.max_input_value_subleaf >= other.eax.max_input_value_subleaf
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
        self.eax.ia32_platform_dca_cap_msr == other.eax.ia32_platform_dca_cap_msr
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
impl LeafDSubleaf0 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl LeafDSubleaf1 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl LeafFSubleaf0 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl LeafFSubleaf1 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf10Subleaf0 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf10Subleaf1 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf10Subleaf3 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf12Subleaf0 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf12Subleaf1 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf12SubleafGt1 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf14Subleaf0 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Leaf14Subleaf1 {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------
// Intel cpuid structure
// -------------------------------------------------------------------------------------------------
/// - Does not support Pentium III processor.
/// - Presumes bit 22 of `IA32_MISC_ENABLE` equals 0.
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
    /// Processor Extended State Enumeration Main Leaf
    pub leaf_d: LeafD,
    /// Intel Resource Director Technology (Intel RDT) Monitoring Enumeration *and*
    /// L3 Cache Intel RDT Monitoring Capability Enumeration
    pub leaf_f: LeafF,
    /// Intel Resource Director Technology (Intel RDT) Allocation Enumeration *and*
    /// L3 Cache Allocation Technology Enumeration *and*
    /// L2 Cache Allocation Technology Enumeration *and*
    /// Memory Bandwidth Allocation Enumeration
    pub leaf_10: Leaf10,
    /// Intel SGX Capability Enumeration *and*
    /// Intel SGX Attributes Enumeration *and*
    /// Intel SGX EPC Enumeration
    pub leaf_12: Leaf12,
    /// Intel Processor Trace Enumeration
    pub leaf_14: Leaf14, /* /// Time Stamp Counter and Nominal Core Crystal Clock Information
                          * pub */
}
impl IntelCpuid {
    #[logfn(Trace)]
    #[logfn_inputs(Info)]
    pub fn supports(&self, other: &Self) -> bool {
        todo!()
        // self.leaf_0.supports(&other.leaf_0) &&
        // self.leaf_1.supports(&other.leaf_1) &&
        // self.leaf_2.supports(&other.leaf_2) &&
        // // TODO leaf 4
        // // self.leaf_4.supports(&other.leaf_4) &&
        // self.leaf_5.supports(&other.leaf_5) &&
        // self.leaf_6.supports(&other.leaf_6) &&
        // self.leaf_7.0.supports(&other.leaf_7.0) &&
        // // TODO leaf 7 subleaf 1
        // self.leaf_9.supports(&other.leaf_9) &&
        // self.leaf_a.supports(&other.leaf_a)
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
        let leaf_7_start = leaf_4_offset + 3;
        // for i in 0..15 {
        //     println!("raw_cpuid[{}].ebx: {}",i,raw_cpuid[i].ebx);
        // }
        debug_assert_eq!(Some(&raw_cpuid[leaf_7_start]), raw_cpuid.get(7, 0));
        let leaf_7_len = raw_cpuid[leaf_7_start].eax as usize;
        dbg!(leaf_7_len);
        debug_assert!(leaf_7_len == 0 || leaf_7_len == 1);
        let leaf7_subleaves = if leaf_7_len == 1 {
            Some(Leaf7Subleaf1::from((
                Leaf7Subleaf1Eax::from(raw_cpuid[leaf_7_start + 1].eax),
                Leaf7Subleaf1Ebx::from(raw_cpuid[leaf_7_start + 1].ebx),
                Leaf7Subleaf1Ecx::from(raw_cpuid[leaf_7_start + 1].ecx),
                Leaf7Subleaf1Edx::from(raw_cpuid[leaf_7_start + 1].edx),
            )))
        } else {
            None
        };
        let leaf_7 = (
            Leaf7Subleaf0::from((
                Leaf7Subleaf0Eax::from(raw_cpuid[leaf_7_start].eax),
                Leaf7Subleaf0Ebx::from(raw_cpuid[leaf_7_start].ebx),
                Leaf7Subleaf0Ecx::from(raw_cpuid[leaf_7_start].ecx),
                Leaf7Subleaf0Edx::from(raw_cpuid[leaf_7_start].edx),
            )),
            leaf7_subleaves,
        );
        let leaf_7_offset = leaf_7_start + leaf_7_len;

        let mut leaf_b_offset = leaf_7_offset + 4;
        dbg!(leaf_b_offset);
        let leaf_b = {
            let mut vec = vec![LeafB::from((
                LeafBEax::from(raw_cpuid[leaf_b_offset].eax),
                LeafBEbx::from(raw_cpuid[leaf_b_offset].ebx),
                LeafBEcx::from(raw_cpuid[leaf_b_offset].ecx),
                LeafBEdx::from(raw_cpuid[leaf_b_offset].edx),
            ))];
            while vec[vec.len() - 1].ecx.level_type != 0u32 {
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
        // dbg!(&raw_cpuid[leaf_b_offset+1]);
        let leaf_d_start = leaf_b_offset + 2;
        // dbg!(&raw_cpuid[leaf_d_start]);
        let mut leaf_d_offset = leaf_d_start + 1;
        // dbg!(&raw_cpuid[leaf_d_offset]);

        // TODO Avoid using `function` to accmulate subleaf d values
        let leaf_d_subleaves = {
            let mut vec = Vec::new();
            while raw_cpuid[leaf_d_offset + 1].function == 13 {
                leaf_d_offset += 1;
                vec.push(LeafDSubleafGt1::from((
                    LeafDSubleafGt1Eax::from(raw_cpuid[leaf_d_offset].eax),
                    LeafDSubleafGt1Ebx::from(raw_cpuid[leaf_d_offset].ebx),
                    LeafDSubleafGt1Ecx::from(raw_cpuid[leaf_d_offset].ecx),
                    LeafDSubleafGt1Edx::from(raw_cpuid[leaf_d_offset].edx),
                )));
            }
            vec
        };

        let leaf_f_start = leaf_d_offset + 2;
        debug_assert_eq!(Some(&raw_cpuid[leaf_f_start]), raw_cpuid.get(15, 0));
        let mut leaf_f_offset = leaf_f_start;
        let leaf_f = {
            let subleaf0 = LeafFSubleaf0::from((
                LeafFSubleaf0Eax::from(raw_cpuid[leaf_f_start].eax),
                LeafFSubleaf0Ebx::from(raw_cpuid[leaf_f_start].ebx),
                LeafFSubleaf0Ecx::from(raw_cpuid[leaf_f_start].ecx),
                LeafFSubleaf0Edx::from(raw_cpuid[leaf_f_start].edx),
            ));
            let subleaf1 = (subleaf0.edx.l3_rdt_monitor == true).then(|| {
                leaf_f_offset += 1;
                LeafFSubleaf1::from((
                    LeafFSubleaf1Eax::from(raw_cpuid[leaf_f_offset].eax),
                    LeafFSubleaf1Ebx::from(raw_cpuid[leaf_f_offset].ebx),
                    LeafFSubleaf1Ecx::from(raw_cpuid[leaf_f_offset].ecx),
                    LeafFSubleaf1Edx::from(raw_cpuid[leaf_f_offset].edx),
                ))
            });
            (subleaf0, subleaf1)
        };
        let leaf_10_start = leaf_f_offset + 1;
        debug_assert_eq!(Some(&raw_cpuid[leaf_10_start]), raw_cpuid.get(16, 0));
        let mut leaf10_offset = leaf_10_start;
        let leaf_10 = {
            let subleaf0 = Leaf10Subleaf0::from((
                Leaf10Subleaf0Eax::from(raw_cpuid[leaf_10_start].eax),
                Leaf10Subleaf0Ebx::from(raw_cpuid[leaf_10_start].ebx),
                Leaf10Subleaf0Ecx::from(raw_cpuid[leaf_10_start].ecx),
                Leaf10Subleaf0Edx::from(raw_cpuid[leaf_10_start].edx),
            ));
            // We use `bool::then` over `bool::then_some` as `bool::then_some` is eagerly
            // evaluated.
            let subleaf1 = (subleaf0.ebx.l3_alloc == true).then(|| {
                leaf10_offset += 1;
                Leaf10Subleaf1::from((
                    Leaf10Subleaf1Eax::from(raw_cpuid[leaf10_offset].eax),
                    Leaf10Subleaf1Ebx::from(raw_cpuid[leaf10_offset].ebx),
                    Leaf10Subleaf1Ecx::from(raw_cpuid[leaf10_offset].ecx),
                    Leaf10Subleaf1Edx::from(raw_cpuid[leaf10_offset].edx),
                ))
            });
            let subleaf2 = (subleaf0.ebx.l2_alloc == true).then(|| {
                leaf10_offset += 1;
                Leaf10Subleaf2::from((
                    Leaf10Subleaf2Eax::from(raw_cpuid[leaf10_offset].eax),
                    Leaf10Subleaf2Ebx::from(raw_cpuid[leaf10_offset].ebx),
                    Leaf10Subleaf2Ecx::from(raw_cpuid[leaf10_offset].ecx),
                    Leaf10Subleaf2Edx::from(raw_cpuid[leaf10_offset].edx),
                ))
            });
            let subleaf3 = (subleaf0.ebx.mem_band_alloc == true).then(|| {
                leaf10_offset += 1;
                Leaf10Subleaf3::from((
                    Leaf10Subleaf3Eax::from(raw_cpuid[leaf10_offset].eax),
                    Leaf10Subleaf3Ebx::from(raw_cpuid[leaf10_offset].ebx),
                    Leaf10Subleaf3Ecx::from(raw_cpuid[leaf10_offset].ecx),
                    Leaf10Subleaf3Edx::from(raw_cpuid[leaf10_offset].edx),
                ))
            });
            (subleaf0, subleaf1, subleaf2, subleaf3)
        };

        let leaf_12_start = leaf10_offset + 2;
        let leaf_12_offset = leaf_12_start;
        debug_assert_eq!(Some(&raw_cpuid[leaf_12_start]), raw_cpuid.get(18, 0));
        let leaf_12 = {
            let subleaf0 = Leaf12Subleaf0::from((
                Leaf12Subleaf0Eax::from(raw_cpuid[leaf_12_start].eax),
                Leaf12Subleaf0Ebx::from(raw_cpuid[leaf_12_start].ebx),
                Leaf12Subleaf0Ecx::from(raw_cpuid[leaf_12_start].ecx),
                Leaf12Subleaf0Edx::from(raw_cpuid[leaf_12_start].edx),
            ));
            // Leaf 12H sub-leaf 1 (ECX = 1) is supported if CPUID.(EAX=07H, ECX=0H):EBX[SGX] = 1.
            dbg!(leaf_7.0.ebx.sgx);
            let (subleaf1, subleaf2) = if leaf_7.0.ebx.sgx == true {
                unimplemented!(
                    "Due to vagueness surrounding the number of subleaves this is currently not \
                     supported"
                );
                // leaf_12_offset += 1;
                // (
                //     Some(Leaf12Subleaf1::from((
                //         Leaf12Subleaf1Eax::from(raw_cpuid[leaf_12_offset].eax),
                //         Leaf12Subleaf1Ebx::from(raw_cpuid[leaf_12_offset].ebx),
                //         Leaf12Subleaf1Ecx::from(raw_cpuid[leaf_12_offset].ecx),
                //         Leaf12Subleaf1Edx::from(raw_cpuid[leaf_12_offset].edx),
                //     ))),
                //     Vec::new(),
                // )
            } else { (None,Vec::new()) };
            // Leaf 12H sub-leaf 2 or higher (ECX >= 2) is supported if
            // CPUID.(EAX=07H, ECX=0H):EBX[SGX] = 1.
            //
            // For sub-leaves (ECX = 2 or higher), definition of EDX,ECX,EBX,EAX[31:4] depends on
            // the sub-leaf type listed below.
            (subleaf0, subleaf1, subleaf2)
        };

        let leaf_14_start = leaf_12_offset + 2;
        let mut leaf_14_offset = leaf_14_start;
        debug_assert_eq!(Some(&raw_cpuid[leaf_14_start]), raw_cpuid.get(19, 0));
        let leaf_14 = {
            let subleaf0 = Leaf14Subleaf0::from((
                Leaf14Subleaf0Eax::from(raw_cpuid[leaf_14_start].eax),
                Leaf14Subleaf0Ebx::from(raw_cpuid[leaf_14_start].ebx),
                Leaf14Subleaf0Ecx::from(raw_cpuid[leaf_14_start].ecx),
                Leaf14Subleaf0Edx::from(raw_cpuid[leaf_14_start].edx),
            ));
            let subleaf1 = match u32::from(&subleaf0.eax.max_subleaf) {
                
                1 => {
                    leaf_14_offset += 1;
                    Some(Leaf14Subleaf1::from((
                        Leaf14Subleaf1Eax::from(raw_cpuid[leaf_14_offset].eax),
                        Leaf14Subleaf1Ebx::from(raw_cpuid[leaf_14_offset].ebx),
                        Leaf14Subleaf1Ecx::from(raw_cpuid[leaf_14_offset].ecx),
                        Leaf14Subleaf1Edx::from(raw_cpuid[leaf_14_offset].edx),
                    )))
                }
                0 => None,
                // TODO Add specific spec version
                _ => unimplemented!(
                    "The Intel specification does not describe subleaves of leaf 14h beyound 1."
                ),
            };
            (subleaf0, subleaf1)
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
            leaf_7,
            leaf_9: Leaf9::from((
                Leaf9Eax::from(raw_cpuid[leaf_7_offset + 2].eax),
                Leaf9Ebx::from(raw_cpuid[leaf_7_offset + 2].ebx),
                Leaf9Ecx::from(raw_cpuid[leaf_7_offset + 2].ecx),
                Leaf9Edx::from(raw_cpuid[leaf_7_offset + 2].edx),
            )),
            leaf_a: LeafA::from((
                LeafAEax::from(raw_cpuid[leaf_7_offset + 3].eax),
                LeafAEbx::from(raw_cpuid[leaf_7_offset + 3].ebx),
                LeafAEcx::from(raw_cpuid[leaf_7_offset + 3].ecx),
                LeafAEdx::from(raw_cpuid[leaf_7_offset + 3].edx),
            )),
            leaf_b,
            leaf_d: (
                LeafDSubleaf0::from((
                    LeafDSubleaf0Eax::from(raw_cpuid[leaf_d_start].eax),
                    LeafDSubleaf0Ebx::from(raw_cpuid[leaf_d_start].ebx),
                    LeafDSubleaf0Ecx::from(raw_cpuid[leaf_d_start].ecx),
                    LeafDSubleaf0Edx::from(raw_cpuid[leaf_d_start].edx),
                )),
                LeafDSubleaf1::from((
                    LeafDSubleaf1Eax::from(raw_cpuid[leaf_d_start + 1].eax),
                    LeafDSubleaf1Ebx::from(raw_cpuid[leaf_d_start + 1].ebx),
                    LeafDSubleaf1Ecx::from(raw_cpuid[leaf_d_start + 1].ecx),
                    LeafDSubleaf1Edx::from(raw_cpuid[leaf_d_start + 1].edx),
                )),
                leaf_d_subleaves,
            ),
            leaf_f,
            leaf_10,
            leaf_12,
            leaf_14,
        }
    }
}
