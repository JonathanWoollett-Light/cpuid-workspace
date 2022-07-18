#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]
#![feature(iter_intersperse)]
use std::collections::HashSet;
use std::fmt::Write;

use proc_macro::{Diagnostic, Level, TokenStream, TokenTree};

/// Procedural macro to generate bit fields.
///
/// ```ignore
/// #[rustfmt::skip]
/// bit_fields::bitfield!(GeneratedBitField,u32,[
///     RANGE1, 0..1,
///     SSE, 2,
///     SSE1 3,
///     RANGE2, 4..6,
///     SSE2, 9,
///     SSE3, 10,
///     RANGE3, 12..15,
///     SSE4, 18
/// ]);
/// let bitfield = GeneratedBitField::from(23548);
/// println!("{}", bitfield);
/// ```
/// Prints:
/// ```test
/// ┌───────┬────────────┬───┬───────┬───────┬────────────┬───┬───────┬───────┬───┬────────────┬───┬───────┬───┐
/// │ Bit/s │    00..=00 │ … │    02 │    03 │    04..=05 │ … │    09 │    10 │ … │    12..=14 │ … │    18 │ … │
/// ├───────┼────────────┼───┼───────┼───────┼────────────┼───┼───────┼───────┼───┼────────────┼───┼───────┼───┤
/// │ Desc  │     RANGE1 │ … │   SSE │  SSE1 │     RANGE2 │ … │  SSE2 │  SSE3 │ … │     RANGE3 │ … │  SSE4 │ … │
/// ├───────┼────────────┼───┼───────┼───────┼────────────┼───┼───────┼───────┼───┼────────────┼───┼───────┼───┤
/// │ Value │          0 │ … │  true │  true │          3 │ … │  true │ false │ … │          5 │ … │ false │ … │
/// └───────┴────────────┴───┴───────┴───────┴────────────┴───┴───────┴───────┴───┴────────────┴───┴───────┴───┘
/// ```
#[proc_macro]
pub fn bitfield(item: TokenStream) -> TokenStream {
    let mut token_stream_iter = item.into_iter();

    // Get struct identifier
    const IDENT_ERR: &str = "1st token must be struct identifier";
    let struct_name = match token_stream_iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(token) => {
            Diagnostic::spanned(token.span(), Level::Error, IDENT_ERR).emit();
            return "".parse().unwrap();
        }
        _ => panic!("{}", IDENT_ERR),
    };
    const TYPE_ERR: &str = "3rd token must be type identifier, options: [u8, u16, u32, u64, u128]";
    let struct_data_type = match token_stream_iter.nth(1) {
        Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
            "u8" | "u16" | "u32" | "u64" | "u128" => ident.to_string(),
            _ => {
                Diagnostic::spanned(ident.span(), Level::Error, TYPE_ERR).emit();
                return "".parse().unwrap();
            }
        },
        Some(token) => {
            Diagnostic::spanned(token.span(), Level::Error, TYPE_ERR).emit();
            return "".parse().unwrap();
        }
        _ => panic!("{}", TYPE_ERR),
    };

    let mut struct_bits = String::new();
    let mut struct_new_bits = String::new();
    let mut bit_index = String::new();
    let bits_len = match struct_data_type.as_str() {
        "u8" => 8,
        "u16" => 16,
        "u32" => 32,
        "u64" => 64,
        "u128" => 128,
        _ => unreachable!(),
    };
    for i in 0u8..bits_len {
        write!(&mut struct_bits, "Bit<{struct_data_type},{i}>,").unwrap();
        struct_new_bits.push_str("Bit(std::marker::PhantomData),");
        write!(
            &mut bit_index,
            "
        impl BitIndex<{struct_data_type},{i}> for {struct_name} {{
            fn bit(&self) -> &Bit<{struct_data_type},{i}> {{
                &self.bits.{i}
            }}
        }}
        impl BitIndexMut<{struct_data_type},{i}> for {struct_name} {{
            fn bit_mut(&mut self) -> &mut Bit<{struct_data_type},{i}> {{
                &mut self.bits.{i}
            }}
        }}
        "
        )
        .unwrap();
    }

    const FIELDS_ERR: &str = "5th token must be an array of types and bit indexes, they must be \
                              ordered non-overlapping, unique and within the bounds of the given \
                              type. e.g. `[FlagOne: 2, FlagTwo: 3, FlagThree: 7, FlagFour: 11]`";
    let mut fields_specific_impl = String::new();
    let mut field_matching_from_hashset = String::new();
    let mut fields_setting_hashset = String::new();
    let mut fields_superset_fn = String::from("true");
    let mut fields_subset_fn = String::from("true");
    let mut fields_disjoint_fn = String::from("false");
    let mut fields_intersection_fn = String::new();
    let mut fields_union_fn = String::new();
    let mut struct_bit_range_definitions = String::new();
    let mut struct_new_ranges = String::new();
    let mut range_specific_impl = String::new();
    let mut range_count = 0;
    // Top border
    // Bit numbers
    // Border
    // Field idents
    // Border
    // Field values
    // Bottom border
    // Fmt values (since write doesnt work with inplace ones)
    let mut display_string = vec![
        String::from("┌───────┬"),
        String::from("│ \x1b[1mBit/s\x1b[0m │"),
        String::from("├───────┼"),
        String::from("│ \x1b[1mDesc\x1b[0m  │"),
        String::from("├───────┼"),
        String::from("│ \x1b[1mValue\x1b[0m │"),
        String::from("└───────┴"),
        String::new(),
    ];
    match token_stream_iter.nth(1) {
        Some(TokenTree::Group(group)) => {
            let fields_stream = group.stream();
            let mut fields_iter = fields_stream.into_iter().peekable();
            let mut pos = 0;
            // eprintln!("got here?");

            let mut pre_existing = HashSet::new();
            loop {
                let next = fields_iter.next();
                // eprintln!("next: {:?}", next);

                let field_ident = match next {
                    Some(TokenTree::Ident(field_ident)) => {
                        let field_ident_str = field_ident.to_string();
                        // If this ident already used
                        if !pre_existing.insert(field_ident_str) {
                            Diagnostic::spanned(
                                field_ident.span(),
                                Level::Error,
                                "Identifier already used",
                            )
                            .emit();
                            return "".parse().unwrap();
                        }
                        field_ident
                    }
                    Some(wrong_field) => {
                        Diagnostic::spanned(wrong_field.span(), Level::Error, "Identifier missing")
                            .emit();
                        return "".parse().unwrap();
                    }
                    None => break,
                };
                // Punct,Literal,Punct,Punct,Literal == Range
                // Punct,Literal,Punct,Literal
                let field_start_pos = match fields_iter.nth(1) {
                    Some(TokenTree::Literal(field_start)) => {
                        let field_start_pos = field_start.to_string().parse::<u8>().unwrap();
                        let mut flag = false;
                        // If position is out of order
                        if field_start_pos < pos {
                            Diagnostic::spanned(
                                field_start.span(),
                                Level::Error,
                                "Position out of order",
                            )
                            .emit();
                            flag = true;
                        }
                        // If position is outside range of provided underlying data type
                        // (u8,u16,etc.)
                        if field_start_pos > bits_len {
                            Diagnostic::spanned(
                                field_start.span(),
                                Level::Error,
                                "Position out of range",
                            )
                            .emit();
                            flag = true;
                        }
                        if flag {
                            return "".parse().unwrap();
                        }
                        // If position has skipped some bits
                        if field_start_pos > pos {
                            let border = "───";
                            display_string[0].push_str(border);
                            display_string[0].push('┬');
                            display_string[1].push_str(" … │");
                            display_string[2].push_str(border);
                            display_string[2].push('┼');
                            display_string[3].push_str(" … │");
                            display_string[4].push_str(border);
                            display_string[4].push('┼');
                            display_string[5].push_str(" … │");
                            display_string[6].push_str(border);
                            display_string[6].push('┴');
                        }
                        // Update order position
                        pos = field_start_pos + 1;

                        field_start
                    }
                    _ => {
                        Diagnostic::spanned(field_ident.span(), Level::Error, "Position missing")
                            .emit();
                        return "".parse().unwrap();
                    }
                };

                let mut add_bit_flags = || {
                    // Set display string
                    let start = field_start_pos.to_string().parse::<u8>().unwrap();
                    let more = start < bits_len;
                    let cropped = field_ident.to_string().chars().take(4).collect::<String>();
                    let border = "───────";
                    display_string[0].push_str(border);
                    display_string[0].push(if more { '┬' } else { '┐' });
                    write!(
                        &mut display_string[1],
                        "    {:02} │",
                        field_start_pos.to_string().parse::<u8>().unwrap()
                    )
                    .unwrap();
                    display_string[2].push_str(border);
                    display_string[2].push(if more { '┼' } else { '┤' });
                    write!(&mut display_string[3], " {cropped:>5} │").unwrap();
                    display_string[4].push_str(border);
                    display_string[4].push(if more { '┼' } else { '┤' });
                    write!(&mut display_string[5], " {{:>5}} │",).unwrap();
                    display_string[6].push_str(border);
                    display_string[6].push(if more { '┴' } else { '┘' });
                    write!(&mut display_string[7], "self.{field_ident}().to_string(),").unwrap();

                    // struct_fields_display_data.push(StructFieldDisplayData::Bit(field_start_pos.
                    // to_string().parse::<u8>().unwrap(),field_ident.to_string()));

                    write!(
                        &mut fields_specific_impl,
                        "
                        pub fn {field_ident}(&self) -> &Bit<{struct_data_type},{field_start_pos}> \
                         {{
                            self.bit::<{field_start_pos}>()
                        }}
                        pub fn {field_ident}_mut(&mut self) -> &mut \
                         Bit<{struct_data_type},{field_start_pos}> {{
                            self.bit_mut::<{field_start_pos}>()
                        }}
                    "
                    )
                    .unwrap();

                    write!(
                        &mut field_matching_from_hashset,
                        "
                        \"{field_ident}\" => {{
                            base.{field_ident}_mut().on();
                        }},
                    "
                    )
                    .unwrap();

                    write!(
                        &mut fields_setting_hashset,
                        "
                        if self.{field_ident}().into() {{
                            set.insert(String::from(\"{field_ident}\"));
                        }}
                    "
                    )
                    .unwrap();

                    write!(
                        &mut fields_superset_fn,
                        "
                        && if other.{field_ident}().into() {{ self.{field_ident}().into() }} else \
                         {{ true }}
                    "
                    )
                    .unwrap();
                    write!(
                        &mut fields_subset_fn,
                        "
                        && if self.{field_ident}().into() {{ other.{field_ident}().into() }} else \
                         {{ true }}
                    "
                    )
                    .unwrap();
                    write!(
                        &mut fields_disjoint_fn,
                        "
                        || !(self.{field_ident}() == other.{field_ident}())
                    "
                    )
                    .unwrap();
                    write!(
                        &mut fields_intersection_fn,
                        "
                        if self.{field_ident}().into() && other.{field_ident}().into() {{
                            base.{field_ident}_mut().on();
                        }}
                    "
                    )
                    .unwrap();
                    write!(
                        &mut fields_union_fn,
                        "
                        if self.{field_ident}().into() || other.{field_ident}().into() {{
                            base.{field_ident}_mut().on();
                        }}
                    "
                    )
                    .unwrap();
                };

                match fields_iter.peek() {
                    // The bit range case
                    Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
                        // Skip what we already checked by peeking
                        fields_iter.next();
                        match (fields_iter.next(), fields_iter.next()) {
                            (
                                Some(TokenTree::Punct(punct2)),
                                Some(TokenTree::Literal(field_end_pos)),
                            ) => {
                                if punct2.as_char() == '.' {
                                    let start = field_start_pos.to_string().parse::<u8>().unwrap();
                                    let end = field_end_pos.to_string().parse::<u8>().unwrap();
                                    if end < start {
                                        Diagnostic::spanned(
                                            field_ident.span(),
                                            Level::Error,
                                            "end < start",
                                        )
                                        .emit();
                                        return "".parse().unwrap();
                                    }
                                    if end > bits_len {
                                        Diagnostic::spanned(
                                            field_ident.span(),
                                            Level::Error,
                                            "end > bits_len",
                                        )
                                        .emit();
                                        return "".parse().unwrap();
                                    }

                                    // Set display string
                                    let more = fields_iter.peek().is_some();
                                    let cropped = field_ident
                                        .to_string()
                                        .chars()
                                        .take(10)
                                        .collect::<String>();
                                    let border = "────────────";
                                    display_string[0].push_str(border);
                                    display_string[0].push(if more { '┬' } else { '┐' });
                                    write!(
                                        &mut display_string[1],
                                        "    {:02}..={:02} │",
                                        start,
                                        end - 1
                                    )
                                    .unwrap();
                                    display_string[2].push_str(border);
                                    display_string[2].push(if more { '┼' } else { '┤' });
                                    write!(&mut display_string[3], " {cropped:>10} │").unwrap();
                                    display_string[4].push_str(border);
                                    display_string[4].push(if more { '┼' } else { '┤' });
                                    write!(&mut display_string[5], " {{:>10}} │",).unwrap();
                                    display_string[6].push_str(border);
                                    display_string[6].push(if more { '┴' } else { '┘' });
                                    write!(
                                        &mut display_string[7],
                                        "self.{field_ident}().to_string(),"
                                    )
                                    .unwrap();

                                    // Add bit range implementations
                                    let type_str =
                                        format!("BitRange<{struct_data_type},{{{start}..{end}}}>");
                                    write!(&mut struct_bit_range_definitions, "{type_str},")
                                        .unwrap();
                                    write!(
                                        &mut struct_new_ranges,
                                        "BitRange(std::marker::PhantomData),"
                                    )
                                    .unwrap();
                                    // range_count += 1;
                                    // eprintln!(".\nrange_count: {}\n.",range_count);
                                    // eprintln!(".\nrange_count: {}\n.",range_count+1);
                                    write!(
                                        &mut range_specific_impl,
                                        "
                                        pub fn {field_ident}(&self) -> &{type_str} {{
                                            &self.ranges.{range_count}
                                        }}
                                        pub fn {field_ident}_mut(&mut self) -> &mut {type_str} 
                                        {{
                                            &mut self.ranges.{range_count}
                                        }}
                                        "
                                    )
                                    .unwrap();
                                    range_count += 1;
                                }
                            }
                            _ => {
                                Diagnostic::spanned(
                                    field_ident.span(),
                                    Level::Error,
                                    "Bit range badly formed",
                                )
                                .emit();
                                return "".parse().unwrap();
                            }
                        }
                    }
                    // The bit flag case
                    Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => add_bit_flags(),
                    None => add_bit_flags(),
                    _ => {
                        Diagnostic::spanned(field_ident.span(), Level::Error, FIELDS_ERR).emit();
                        return "".parse().unwrap();
                    }
                }
                // We skip the punctuation for the next iteration.
                fields_iter.next();
            }
            // If position of last mapped area is before end of underlying data
            if pos < bits_len {
                let border = "───";
                display_string[0].push_str(border);
                display_string[0].push('┐');
                display_string[1].push_str(" … │");
                display_string[2].push_str(border);
                display_string[2].push('┤');
                display_string[3].push_str(" … │");
                display_string[4].push_str(border);
                display_string[4].push('┤');
                display_string[5].push_str(" … │");
                display_string[6].push_str(border);
                display_string[6].push('┘');
            }
        }
        None => {}
        _ => panic!("5th token should be group of bit flags and bit ranges"),
    };

    let display_full_string_fmt_values = display_string.pop().unwrap();
    let layout = format!(
        "#[cfg_attr(feature = \"serde\", derive(serde::Serialize,serde::Deserialize))]
        #[derive(Clone)]
        pub struct {struct_name} {{
            pub data: {struct_data_type},
            ranges: ({struct_bit_range_definitions}),
            bits: ({struct_bits})
        }}
        // We cannot derive [`std::fmt::Debug`] as `self.bits` has too many elements.
        impl std::fmt::Debug for {struct_name} {{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                f.debug_struct(\"{struct_name}\")
                    .field(\"data\",&self.data)
                    .finish()
            }}
        }}
        impl std::fmt::Display for {struct_name} {{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                write!(f,\"\n{display_full_string}\",{display_full_string_fmt_values})
            }}
        }}
        impl std::fmt::Binary for {struct_name} {{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                fmt::Binary::fmt(&self.data, f)
            }}
        }}
        impl<T:std::fmt::Display> std::convert::TryFrom<std::collections::HashSet<T>> for {struct_name} {{
            type Error = &'static str;
            fn try_from(set: std::collections::HashSet<T>) -> Result<Self,Self::Error> {{
                let mut base = Self::from(0);
                for key in set.into_iter() {{
                    match key.to_string().as_str() {{
                        {field_matching_from_hashset}
                        _ => return Err(\"Non-specified flag found in given set\")
                    }};
                }}
                Ok(base)
            }}
        }}
        {into_hashset}
        /// Constructs `self` with the given internal value.
        impl std::convert::From<{struct_data_type}> for {struct_name} {{
            fn from(data: {struct_data_type}) -> Self {{
                Self {{
                    data,
                    ranges: ({struct_new_ranges}),
                    bits: ({struct_new_bits}),
                }}
            }}
        }}
        impl {struct_name} {{
            /// Returns if `self` is a [`superset`](https://en.wikipedia.org/wiki/Subset) of `other`.
            pub fn superset(&self, other: &Self) -> bool {{
                {fields_superset_fn}
            }}
            /// Returns if `self` is a [`subset`](https://en.wikipedia.org/wiki/Subset) of `other`.
            pub fn subset(&self, other: &Self) -> bool {{
                {fields_subset_fn}
            }}
            /// Returns if `self` and `other` are [`disjoint sets`](https://en.wikipedia.org/wiki/Disjoint_sets).
            pub fn disjoint(&self, other: &Self) -> bool {{
                {fields_disjoint_fn}
            }}
            /// Returns the [`intersection`](https://en.wikipedia.org/wiki/Intersection_(set_theory)) of `self` and `other`.
            pub fn intersection(&self, other: &Self) -> Self {{
                let mut base = Self::from(0);
                {fields_intersection_fn}
                base
            }}
            /// Returns the [`union`](https://en.wikipedia.org/wiki/Union_(set_theory)) of `self` and `other`.
            pub fn union(&self, other: &Self) -> Self {{
                let mut base = Self::from(0);
                {fields_union_fn}
                base
            }}
            /// Returns a reference to the `N`th bit.
            pub fn bit<const N: usize>(&self) -> &Bit<{struct_data_type},N>
            where
                Self: BitIndex<{struct_data_type},N>,
            {{
                <Self as BitIndex<{struct_data_type},N>>::bit(self)
            }}
            /// Returns a mutable reference to the `N`th bit.
            pub fn bit_mut<const N: usize>(&mut self) -> &mut Bit<{struct_data_type},N>
            where
                Self: BitIndexMut<{struct_data_type},N>,
            {{
                <Self as BitIndexMut<{struct_data_type},N>>::bit_mut(self)
            }}
            {fields_specific_impl}
            {range_specific_impl}
        }}
        {bit_index}
        ", into_hashset = if !struct_bit_range_definitions.is_empty() { String::new() } else { format!("
            #[allow(clippy::from_over_into)]
            impl std::convert::Into<std::collections::HashSet<String>> for {struct_name} {{
                fn into(self) -> std::collections::HashSet<String> {{
                    let mut set = std::collections::HashSet::new();
                    {fields_setting_hashset}
                    set
                }}
            }}
        ")},display_full_string = {
            display_string.into_iter().intersperse(String::from("\n")).collect::<String>()
        }
    );
    // eprintln!("layout: {}", layout);
    // "fn answer() -> u32 { 42 }".parse().unwrap()
    layout.parse().unwrap()
}
