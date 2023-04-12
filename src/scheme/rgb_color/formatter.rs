use crate::{
    scheme::rgb_color::RgbColor,
    template::color_field::{Dec, Format, Hex, Hsl, HslFormatter, Rgb},
};
use ramhorns::{encoding::Encoder, Content};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RgbColorFormatter {
    pub color: RgbColor,
    pub format: Format,
}

impl Display for RgbColorFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let RgbColor([r, g, b]) = self.color;

        match self.format {
            Format::Hex(Hex::Rgb) => write!(f, "{r:02x}{g:02x}{b:02x}"),
            Format::Hex(Hex::R) => write!(f, "{r:02x}"),
            Format::Hex(Hex::G) => write!(f, "{g:02x}"),
            Format::Hex(Hex::B) => write!(f, "{b:02x}"),
            Format::Hex(Hex::Bgr) => write!(f, "{b:02x}{g:02x}{r:02x}"),

            Format::Rgb(Rgb::R) => write!(f, "{r}"),
            Format::Rgb(Rgb::G) => write!(f, "{g}"),
            Format::Rgb(Rgb::B) => write!(f, "{b}"),

            Format::Dec(Dec::R) => write!(f, "{:.2}", r as f64 / 255.0),
            Format::Dec(Dec::G) => write!(f, "{:.2}", g as f64 / 255.0),
            Format::Dec(Dec::B) => write!(f, "{:.2}", b as f64 / 255.0),

            Format::Hsl(Hsl::H) => write!(f, "{:.2}", HslFormatter::from_color(self.color).hue),
            Format::Hsl(Hsl::L) => {
                write!(f, "{:.2}", HslFormatter::from_color(self.color).luminance)
            }
            Format::Hsl(Hsl::S) => {
                write!(f, "{:.2}", HslFormatter::from_color(self.color).saturation)
            }
        }
    }
}

impl Content for RgbColorFormatter {
    fn is_truthy(&self) -> bool {
        true
    }

    fn render_escaped<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.format_escaped(self)
    }
}
