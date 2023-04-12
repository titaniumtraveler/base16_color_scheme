use serde::{Deserialize, Serialize};

pub use formatter::RgbColorFormatter;

mod formatter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RgbColor(pub [u8; 3]);

impl Serialize for RgbColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        hex::serialize(self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for RgbColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        hex::deserialize(deserializer).map(RgbColor)
    }
}
