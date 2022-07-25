use bit_fields::bitfield;
bitfield!(GeneratedBitField,u8,{
        RANGE1: 0..1,
        SSE: 2,
        SSE1: 3,
        RANGE2: 4..6,
});

fn main() {}
