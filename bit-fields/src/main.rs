bit_fields::bitfield!(LeafDSubleaf1Ecx, u32, {
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

fn main() {}
