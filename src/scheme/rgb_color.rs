use serde::{Deserialize, Serialize};

pub use formatter::RgbColorFormatter;

mod formatter;

/// type representing a hex rgb color, which implements [`Serialize`](serde::Serialize)/[`Deserialze`](serde::Deserialize)
///
/// it uses [`hex`] for the implementation so it's deserialization is case insensitive and it's
/// serialization is lower case by default.
///
/// # Examples
///
/// ```rust
/// use base16_color_scheme::scheme::RgbColor;
///
/// let color: RgbColor = serde_yaml::from_str("7CaFc2").unwrap();
/// assert_eq!(color, RgbColor([0x7c, 0xaf, 0xc2]));
/// assert_eq!(serde_yaml::to_string(&color).unwrap(), "7cafc2\n");
/// ```
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
