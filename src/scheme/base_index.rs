use hex::FromHex;
use serde::{de::Visitor, Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// type representing a base16 colorscheme key
///
/// [`BaseIndex`] has a custom [`Serialize`](serde::Serialize)/[`Deserialze`](serde::Deserialize) to
/// turn `base07` to `BaseIndex(0x07)` and back.
///
/// # Examples
///
/// ```rust
/// use base16_color_scheme::scheme::{BaseIndex, RgbColor};
/// use std::collections::BTreeMap;
///
/// let map: BTreeMap<BaseIndex, RgbColor> = serde_yaml::from_str("base07: 7cafc2").unwrap();
/// assert_eq!(
///     map,
///     BTreeMap::from([(BaseIndex(0x07), RgbColor([0x7c, 0xaf, 0xc2]))])
/// );
/// ```
///
/// this is pretty much what happens in [`Scheme::colors`](crate::Scheme::colors).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseIndex(pub u8);

impl Serialize for BaseIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format_args!("base{:02X}", self.0))
    }
}

struct BaseIndexVisitor;

impl<'de> Deserialize<'de> for BaseIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(BaseIndexVisitor)
    }
}

impl<'de> Visitor<'de> for BaseIndexVisitor {
    type Value = BaseIndex;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a hex number between 00 and FF")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(number) = v.strip_prefix("base") {
            FromHex::from_hex(number)
                .map(|[v]: [u8; 1]| BaseIndex(v))
                .map_err(E::custom)
        } else {
            Err(E::custom(format!(r#"should start with "base""#)))
        }
    }
}
