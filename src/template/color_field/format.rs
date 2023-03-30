use crate::scheme::RgbColor;
use ramhorns::{encoding::Encoder, Content};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct RgbColorFormatter {
    color: RgbColor,
    format: Format,
}

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Format {
    Hex(Hex),
    Rgb(Rgb),
    Dec(Dec),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hex {
    Rgb,
    R,
    G,
    B,
    Bgr,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rgb {
    R,
    G,
    B,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dec {
    R,
    G,
    B,
}

impl Content for RgbColorFormatter {
    fn is_truthy(&self) -> bool {
        true
    }

    fn render_escaped<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        let RgbColor([r, g, b]) = self.color;

        match self.format {
            Format::Hex(Hex::Rgb) => encoder.format_escaped(format_args!("{r:02x}{g:02x}{b:02x}")),
            Format::Hex(Hex::R) => encoder.format_escaped(format_args!("{r:02x}")),
            Format::Hex(Hex::G) => encoder.format_escaped(format_args!("{g:02x}")),
            Format::Hex(Hex::B) => encoder.format_escaped(format_args!("{b:02x}")),
            Format::Hex(Hex::Bgr) => encoder.format_escaped(format_args!("{b:02x}{g:02x}{r:02x}")),

            Format::Rgb(Rgb::R) => encoder.format_escaped(format_args!("{r}")),
            Format::Rgb(Rgb::G) => encoder.format_escaped(format_args!("{g}")),
            Format::Rgb(Rgb::B) => encoder.format_escaped(format_args!("{b}")),

            Format::Dec(Dec::R) => encoder.format_escaped(format_args!("{:.2}", r as f64 / 255.0)),
            Format::Dec(Dec::G) => encoder.format_escaped(format_args!("{:.2}", g as f64 / 255.0)),
            Format::Dec(Dec::B) => encoder.format_escaped(format_args!("{:.2}", b as f64 / 255.0)),
        }
    }
}
