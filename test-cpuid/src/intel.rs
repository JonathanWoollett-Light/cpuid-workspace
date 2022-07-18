
use bit_fields::bitfield;
use super::FixedString;

#[rustfmt::skip]
bitfield!(Leaf1Eax, u32, [
    stepping_id, 0..4,
    model, 4..8,
    family_id, 8..12,
    processor_type, 12..14,
    extended_model_id: 16..20,
    extended_family_id: 20..28,
]);
#[rustfmt::skip]
bitfield!(Leaf1Ebx, u32, [
    brand_index, 0..8,
    clflush, 8..16,
    // maximum_number_addressable_ids_for_logical_processors_in_this_physical_package
    max_addressable_logical_processor_ids: 16..24,
    initial_apic_id: 24..32,
]);
#[rustfmt::skip]
bitfield!(Leaf1Ecx, u32, [
    sse3, 0, pclmulqdq, 1, dtes64, 2, monitor, 3, ds_cpl, 4, vmx, 5, smx, 6, eist, 7, tm2, 8,
    ssse3, 9, cnxt_id, 10, sdbg, 11, fma, 12, cmpxchg16b, 13, xtpr_update_control, 14, pdcm, 15,
    // Reserved
    pcid, 17, dca, 18, sse4_1, 19, sse4_2, 20, x2apic, 21, movbe, 22, popcnt, 23, tsc_deadline, 24,
    aesni, 25, xsave, 26, osxsave, 27,avx, 28,f16c, 29, rdrand, 30,
    // Not used
]);
#[rustfmt::skip]
bitfield!(Leaf1Edx, u32, [
    fpu, 0, vme, 1, de, 2, pse, 3, tsc, 4, msr, 5, pae, 6, mce, 7, cx8, 8, apic, 9,
    // Reserved
    sep, 11, mtrr, 12, pge, 13, mca, 14, cmov, 15, pat, 16, pse3_36, 17, psn, 18, clfsh, 19,
    // Reserved
    ds, 21, acpi, 22, mmx, 23, fxsr, 24, sse, 25, sse2, 26, ss, 27, htt, 28, tm, 29,
    // Reserved
    pbe, 31,
]);

struct Leaf<A,B,C,D> {
    eax: A,
    ebx: B,
    ecx: C,
    edX: D 
}
struct IntelCpuid {
    leaf0: Leaf<u32,FixedString<4>,FixedString<4>,FixedString<4>>,
    leaf1: Leaf<Leaf1Eax,Leaf1Ebx,Leaf1Ecx,Leaf1Edx>
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    #[test]
    fn intel() {
        
    }
}