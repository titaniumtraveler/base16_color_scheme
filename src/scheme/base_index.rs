use hex::FromHex;
use serde::{de::Visitor, Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
