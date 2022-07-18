use crate::bitflags_serde;

bitflags_serde!(Leaf0x1_SubLeaf0_Ecx, a);
bitflags_serde!(Leaf0x1_SubLeaf0_Edx, b);
bitflags_serde!(Leaf0x6_SubLeaf0_Eax, c);
bitflags_serde!(Leaf0x6_SubLeaf0_Ecx, d);
bitflags_serde!(Leaf0x7_SubLeaf0_Ebx, e);
bitflags_serde!(Leaf0x7_SubLeaf0_Ecx, f);
bitflags_serde!(Leaf0x7_SubLeaf0_Edx, g);
bitflags_serde!(Leaf0x7_SubLeaf1_Eax, h);

bitflags_serde!(Leaf0xD_SubLeaf1_Eax, i);
bitflags_serde!(Leaf0x12_SubLeaf0_Eax, j);
bitflags_serde!(Leaf0x14_SubLeaf0_Ebx, k);
bitflags_serde!(Leaf0x19_SubLeaf0_Ebx, l);

bitflags_serde!(Leaf0x8000_0001_SubLeaf0_Edx, m);
bitflags_serde!(Leaf0x8000_0001_SubLeaf0_Ecx, n);

bitflags_serde!(Leaf0x8000_0008_SubLeaf0_Ebx, o);

bitflags_serde!(Leaf0x8000_001F_SubLeaf0_Eax, p);

const NIBBLE_SEPARATOR: char = '_';

/// Internal macro for serde bit flag implementations.
#[macro_export]
macro_rules! bitflags_serde {
    ( $x:ident, $mod:ident ) => {
        pub mod $mod {
            use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
            use $crate::$x;
            type Flags = $x;

            pub fn serialize<S>(date: &Flags, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                // We format the bits in binary
                let mut base = format!("{:032b}", date.bits());
                // We insert a nibble separator
                // TODO Use https://doc.rust-lang.org/std/iter/struct.Intersperse.html when
                // stabilized.
                let mut offset = 0;
                for i in (4..32).step_by(4) {
                    base.insert(i + offset, $crate::bitflags_util::NIBBLE_SEPARATOR);
                    offset += 1;
                }
                base.serialize(serializer)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<Flags, D::Error>
            where
                D: Deserializer<'de>,
            {
                let raw = String::deserialize(deserializer)?;
                // Removes nibble separator
                let replaced =
                    raw.replace(&$crate::bitflags_util::NIBBLE_SEPARATOR.to_string(), "");
                let number = u32::from_str_radix(&replaced, 2)
                    .map_err(|_| serde::de::Error::custom("radix fail"))?;

                // We use `from_bits_unchecked` over `from_bits` here as this allows unlabelled bits
                // to be active. A user may set an unspecified reserved bit for some specific use
                // case, this allows that.
                Ok(unsafe { $x::from_bits_unchecked(number) })
            }
        }
    };
}

pub mod processor_version_information_mod {
    use std::collections::HashMap;

    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    use crate::ProcessorVersionInformation;
    type Flags = ProcessorVersionInformation;

    pub fn serialize<S>(date: &Flags, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = [
            ("stepping_id", date.stepping_id()),
            ("model", date.model()),
            ("family_id", date.family_id()),
            ("processor_type", date.processor_type()),
            ("extended_model_id", date.extended_model_id()),
            ("extended_family_id", date.extended_family_id()),
        ]
        .into_iter()
        .collect::<HashMap<&str, u8>>();
        map.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Flags, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = HashMap::<&str, u8>::deserialize(deserializer)?;
        ProcessorVersionInformation::try_from(raw)
            .map_err(|_| serde::de::Error::custom("Unexpected flags value {:?}"))
    }
}

pub mod leaf0x8000_0008_sub_leaf0_eax_mod {
    use std::collections::HashMap;

    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    use crate::Leaf0x8000_0008_SubLeaf0_Eax;
    type Flags = Leaf0x8000_0008_SubLeaf0_Eax;

    pub fn serialize<S>(date: &Flags, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = [
            (
                "number_of_physical_address_bits",
                date.number_of_physical_address_bits(),
            ),
            (
                "number_of_linear_address_bits",
                date.number_of_linear_address_bits(),
            ),
        ]
        .into_iter()
        .collect::<HashMap<&str, u8>>();
        map.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Flags, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = HashMap::<&str, u8>::deserialize(deserializer)?;
        Leaf0x8000_0008_SubLeaf0_Eax::try_from(raw)
            .map_err(|_| serde::de::Error::custom("Unexpected flags value {:?}"))
    }
}

pub mod leaf0x8000_0008_sub_leaf0_ecx_mod {
    use std::collections::HashMap;

    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    use crate::Leaf0x8000_0008_SubLeaf0_Ecx;
    type Flags = Leaf0x8000_0008_SubLeaf0_Ecx;

    pub fn serialize<S>(date: &Flags, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = [
            (
                "number_of_physical_cores_minus_1",
                date.number_of_physical_cores_minus_1(),
            ),
            ("log2_of_maximum_apic_id", date.log2_of_maximum_apic_id()),
            (
                "performance_timestamp_counter_size",
                date.performance_timestamp_counter_size(),
            ),
        ]
        .into_iter()
        .collect::<HashMap<&str, u8>>();
        map.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Flags, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = HashMap::<&str, u8>::deserialize(deserializer)?;
        Leaf0x8000_0008_SubLeaf0_Ecx::try_from(raw)
            .map_err(|_| serde::de::Error::custom("Unexpected flags value {:?}"))
    }
}
