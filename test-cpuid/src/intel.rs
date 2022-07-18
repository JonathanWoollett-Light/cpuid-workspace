use bit_fields::bitfield;
use super::FixedString;

#[rustfmt::skip]
bitfield!(Leaf1Eax, u32, [
    stepping_id, 0..4,
    model,4..8,
    family_id,8..12,
    processor_type,12..14,
]);
struct Leaf<A,B,C,D> {
    eax: A,
    ebx: B,
    ecx: C,
    edX: D 
}
struct IntelCpuid {
    leaf0: Leaf<u32,FixedString<3>,FixedString<3>,FixedString<3>>,
    // leaf1: Leaf<>
}