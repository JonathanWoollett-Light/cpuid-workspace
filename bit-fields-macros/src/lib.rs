#![warn(clippy::pedantic)]
use std::collections::HashSet;
use std::fmt::Write;

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

// TODO Allow writing rustdoc comments on structs

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
///
/// # Panics
///
/// For a whole load of reason.
#[allow(clippy::too_many_lines)]
#[proc_macro]
pub fn bitfield(item: TokenStream) -> TokenStream {
    const IDENT_ERR: &str = "1st token must be struct identifier";
    const TYPE_ERR: &str = "3rd token must be type identifier, options: [u8, u16, u32, u64, u128]";
    const FIELDS_ERR: &str = "5th token must be an array of types and bit indexes, they must be \
                              ordered non-overlapping, unique and within the bounds of the given \
                              type. e.g. `[FlagOne: 2, FlagTwo: 3, FlagThree: 7, FlagFour: 11]`";

    // eprintln!("item: {:#?}",item);
    // panic!("stop here");

    let mut token_stream_iter = item.into_iter();

    // Get struct identifier
    let struct_name = match token_stream_iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(token) => return diagnostic(token.span(), IDENT_ERR),
        _ => panic!("{}", IDENT_ERR),
    };

    let struct_data_type = match token_stream_iter.nth(1) {
        Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
            "u8" | "u16" | "u32" | "u64" | "u128" => ident.to_string(),
            _ => return diagnostic(ident.span(), TYPE_ERR),
        },
        Some(token) => return diagnostic(token.span(), TYPE_ERR),
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
        write!(&mut struct_bits, "bit_fields::Bit<{struct_data_type},{i}>,").unwrap();
        struct_new_bits.push_str("bit_fields::Bit(std::marker::PhantomData),");
        write!(
            &mut bit_index,
            "
        impl bit_fields::BitIndex<{struct_data_type},{i}> for {struct_name} {{
            fn bit(&self) -> &bit_fields::Bit<{struct_data_type},{i}> {{
                &self.bits.{i}
            }}
        }}
        impl bit_fields::BitIndexMut<{struct_data_type},{i}> for {struct_name} {{
            fn bit_mut(&mut self) -> &mut bit_fields::Bit<{struct_data_type},{i}> {{
                &mut self.bits.{i}
            }}
        }}
        "
        )
        .unwrap();
    }

    let mut field_matching_from_hashset = String::new();
    let mut fields_setting_hashset = String::new();
    let mut fields_superset_fn = String::from("true");
    let mut fields_subset_fn = String::from("true");
    let mut fields_disjoint_fn = String::from("false");
    let mut fields_intersection_fn = String::new();
    let mut fields_union_fn = String::new();
    let mut struct_bit_range_definitions = String::new();
    let mut struct_doc_table_layout =
        String::from("/// \t<tr><th>Bit/s</th><th>Identifier</th><th>Descripton</th></tr>\n");
    let mut struct_member_fields = String::new();
    let mut struct_member_fields_initialization = String::new();
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
    // Skip seperator
    let group = match token_stream_iter.nth(1) {
        Some(TokenTree::Group(group)) => group,
        None => Group::new(Delimiter::None, TokenStream::new()),
        Some(other) => {
            return diagnostic(
                other.span(),
                "5th token should be group of bit flags and bit ranges",
            )
        }
    };

    let fields_stream = group.stream();
    let mut fields_iter = fields_stream.into_iter().peekable();
    let mut pos = 0;
    // eprintln!("got here?");

    let mut pre_existing = HashSet::new();
    let mut rustdoc = String::new();
    loop {
        eprintln!("rustdoc: {}", rustdoc);
        let next = fields_iter.next();
        eprintln!("next: {:?}", next);
        let field_ident = match next {
            Some(TokenTree::Punct(doc_comment_punct)) if doc_comment_punct.as_char() == '#' => {
                eprintln!("doc_comment_punct: {:?}", doc_comment_punct);
                if let Some(TokenTree::Group(doc_group)) = fields_iter.next() {
                    eprintln!("doc_group: {:?}", doc_group);
                    if let Some(TokenTree::Literal(doc_comment_comment)) =
                        doc_group.stream().into_iter().nth(2)
                    {
                        eprintln!("doc_comment_comment: {:?}", doc_comment_comment);
                        let temp = doc_comment_comment.to_string();
                        // Remove " from start and end (TODO Do this better)
                        let temp = temp
                            .chars()
                            .skip(1)
                            .take(temp.len() - 2)
                            .collect::<String>();
                        // Trim space of front e.g. `/// abcde` produces `" abcde"` and we want
                        // `abcde`
                        let temp = temp.trim_start();
                        rustdoc.push_str(temp);
                        rustdoc.push(' ');
                        continue;
                    }
                    return diagnostic(
                        doc_group.span(),
                        "expected rustdoc comment within `#` group",
                    );
                }
                return diagnostic(
                    doc_comment_punct.span(),
                    "expected rustdoc comment following `#`",
                );
            }
            Some(TokenTree::Ident(field_ident)) => {
                let field_ident_str = field_ident.to_string();
                // If this ident already used
                if !pre_existing.insert(field_ident_str) {
                    return diagnostic(field_ident.span(), "Identifier already used");
                }
                field_ident
            }
            Some(wrong_field) => return diagnostic(wrong_field.span(), "Identifier missing"),
            None => break,
        };
        eprintln!("field_ident: {:?}", field_ident);
        // Punct,Literal,Punct,Punct,Literal == Range
        // Punct,Literal,Punct,Literal
        let field_start_pos = match fields_iter.nth(1) {
            Some(TokenTree::Literal(field_start)) => {
                let field_start_pos = field_start.to_string().parse::<u8>().unwrap();
                // If position is out of order
                if field_start_pos < pos {
                    return diagnostic(field_start.span(), "Position out of order");
                }
                // If position is outside range of provided underlying data type
                // (u8,u16,etc.)
                if field_start_pos > bits_len {
                    return diagnostic(field_start.span(), "Position out of range");
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
            _ => return diagnostic(field_ident.span(), "Position missing"),
        };
        eprintln!("field_start_pos: {:?}", field_start_pos);

        let mut add_bit_flags = || {
            // Set display string
            let start = field_start_pos.to_string().parse::<u8>().unwrap();
            let more = start < bits_len - 1;
            let cropped = field_ident.to_string().chars().take(4).collect::<String>();
            let border = "───────";
            display_string[0].push_str(border);
            display_string[0].push(if more { '┬' } else { '┐' });
            write!(&mut display_string[1], "    {start:02} │",).unwrap();
            display_string[2].push_str(border);
            display_string[2].push(if more { '┼' } else { '┤' });
            write!(&mut display_string[3], " {cropped:>5} │").unwrap();
            display_string[4].push_str(border);
            display_string[4].push(if more { '┼' } else { '┤' });
            write!(&mut display_string[5], " {{:>5}} │",).unwrap();
            display_string[6].push_str(border);
            display_string[6].push(if more { '┴' } else { '┘' });
            write!(&mut display_string[7], "self.{field_ident}.to_string(),").unwrap();

            writeln!(
                &mut struct_member_fields,
                "/// {rustdoc}\npub {field_ident}: \
                 bit_fields::Bit<{struct_data_type},{field_start_pos}>,"
            )
            .unwrap();

            writeln!(
                &mut struct_doc_table_layout,
                "/// \t<tr><td>{start:02}</td><td>{}</td><td>{}</td></tr>",
                field_ident, rustdoc
            )
            .unwrap();
            rustdoc.clear();

            writeln!(
                &mut struct_member_fields_initialization,
                "{field_ident}: bit_fields::Bit(std::marker::PhantomData),"
            )
            .unwrap();

            write!(
                &mut field_matching_from_hashset,
                "
                \"{field_ident}\" => {{
                    base.{field_ident}.on();
                }},
            "
            )
            .unwrap();

            write!(
                &mut fields_setting_hashset,
                "
                if self.{field_ident} == true {{
                    set.insert(String::from(\"{field_ident}\"));
                }}
            "
            )
            .unwrap();

            write!(
                &mut fields_superset_fn,
                "
                && if other.{field_ident} == true {{ bool::from(&self.{field_ident}) }} else {{ \
                 true }}
            "
            )
            .unwrap();
            write!(
                &mut fields_subset_fn,
                "
                && if self.{field_ident} == true {{ bool::from(&other.{field_ident}) }} else {{ \
                 true }}
            "
            )
            .unwrap();
            write!(
                &mut fields_disjoint_fn,
                "
                || !(self.{field_ident} == other.{field_ident})
            "
            )
            .unwrap();
            write!(
                &mut fields_intersection_fn,
                "
                if self.{field_ident} == true && other.{field_ident} == true {{
                    base.{field_ident}.on();
                }}
            "
            )
            .unwrap();
            write!(
                &mut fields_union_fn,
                "
                if self.{field_ident} == true || other.{field_ident} == true {{
                    base.{field_ident}.on();
                }}
            "
            )
            .unwrap();
        };
        // To check whether the field is a bit flag or bit field we check if the next token is `.`
        // (which indicates a range)
        match fields_iter.peek() {
            // The bit range case
            Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
                // Skip what we already checked by peeking
                fields_iter.next();
                match (fields_iter.next(), fields_iter.next()) {
                    (Some(TokenTree::Punct(punct2)), Some(TokenTree::Literal(field_end_pos))) => {
                        if punct2.as_char() == '.' {
                            let start = field_start_pos.to_string().parse::<u8>().unwrap();
                            let end = field_end_pos.to_string().parse::<u8>().unwrap();
                            if end < start {
                                return diagnostic(field_ident.span(), "end < start");
                            }
                            if end > bits_len {
                                return diagnostic(field_ident.span(), "end > bits_len");
                            }

                            // Set display string
                            // TODO With 1 bitrange defined in struct, print will not work
                            // correctly, fix that.
                            let more = fields_iter.peek().is_some();
                            let cropped =
                                field_ident.to_string().chars().take(10).collect::<String>();
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
                            write!(&mut display_string[7], "self.{field_ident}.to_string(),")
                                .unwrap();

                            // Add bit range implementations
                            let type_str =
                                format!("bit_fields::BitRange<{struct_data_type},{start},{end}>");
                            writeln!(
                                &mut struct_member_fields,
                                "/// {rustdoc}\npub {field_ident}: {type_str},"
                            )
                            .unwrap();

                            writeln!(
                                &mut struct_doc_table_layout,
                                "/// \t<tr><td>{:02}..={:02}</td><td>{}</td><td>{}</td></tr>",
                                start,
                                end - 1,
                                field_ident,
                                rustdoc
                            )
                            .unwrap();
                            rustdoc.clear();

                            writeln!(
                                &mut struct_member_fields_initialization,
                                "{field_ident}: bit_fields::BitRange(std::marker::PhantomData),"
                            )
                            .unwrap();

                            write!(&mut struct_bit_range_definitions, "{type_str},").unwrap();
                        }
                    }
                    _ => return diagnostic(field_ident.span(), "Bit range badly formed"),
                }
            }
            // The bit flag case
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => add_bit_flags(),
            None => add_bit_flags(),
            _ => return diagnostic(field_ident.span(), FIELDS_ERR),
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

    let display_full_string_fmt_values = display_string.pop().unwrap();
    let layout = format!("\
        /// An {bits_len} bit structure containing a number of bit flags and bit fields.
        ///
        /// ## Layout
        ///
        /// <table>
        {struct_doc_table_layout}
        /// </table>
        #[cfg_attr(feature = \"serde\", derive(serde::Serialize,serde::Deserialize))]
        #[derive(Clone)]
        pub struct {struct_name} {{
            pub data: {struct_data_type},
            pub bits: ({struct_bits}),
            {struct_member_fields}
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
                write!(f,\"{display_full_string}\",{display_full_string_fmt_values})
            }}
        }}
        impl std::fmt::Binary for {struct_name} {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                std::fmt::Binary::fmt(&self.data, f)
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
                    bits: ({struct_new_bits}),
                    {struct_member_fields_initialization}
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
            pub fn bit<const N: u8>(&self) -> &bit_fields::Bit<{struct_data_type},N>
            where
                Self: bit_fields::BitIndex<{struct_data_type},N>,
            {{
                <Self as bit_fields::BitIndex<{struct_data_type},N>>::bit(self)
            }}
            /// Returns a mutable reference to the `N`th bit.
            pub fn bit_mut<const N: u8>(&mut self) -> &mut bit_fields::Bit<{struct_data_type},N>
            where
                Self: bit_fields::BitIndexMut<{struct_data_type},N>,
            {{
                <Self as bit_fields::BitIndexMut<{struct_data_type},N>>::bit_mut(self)
            }}
        }}
        {bit_index}
        ", into_hashset = if struct_bit_range_definitions.is_empty() { format!("
            // TODO Make this into a `From` implementation
            #[allow(clippy::from_over_into)]
            impl std::convert::Into<std::collections::HashSet<String>> for {struct_name} {{
                fn into(self) -> std::collections::HashSet<String> {{
                    let mut set = std::collections::HashSet::new();
                    {fields_setting_hashset}
                    set
                }}
            }}
        ")} else { String::new() },display_full_string = {
            display_string.into_iter().map(|s|format!("\n{}",s)).collect::<String>()
        }
    );
    // eprintln!("layout: {}", layout);
    // "fn answer() -> u32 { 42 }".parse().unwrap()
    layout.parse().unwrap()
}
fn diagnostic(_span: proc_macro::Span, message: &str) -> proc_macro::TokenStream {
    // It is preferable to use`proc_macro::Diagnostic` we should switch this when
    // `proc_macro::Diagnostic` is stabilized.
    // proc_macro::Diagnostic::spanned($span, proc_macro:: $message).emit();
    // return proc_macro::TokenStream::new();
    panic!("{}", message);
}
/// Splits a line of text at spaces into lines such that each line is closest to but does not exceed
/// the given line length
#[allow(dead_code)]
fn split_space(s: &str, l: usize) -> Vec<String> {
    eprintln!("split_space start");
    let chars = s.chars().collect::<Vec<_>>();

    let mut i = 0;
    let mut lines = Vec::new();
    while i + l < chars.len() {
        for line_len in (0..l).rev() {
            let n = i + line_len;
            if chars[n] == ' ' {
                lines.push(chars[i..n].iter().collect::<String>());
                i = n + 1;
                break;
            }
        }
    }
    lines.push(chars[i..].iter().collect::<String>());
    eprintln!("split_space end");

    lines
}
