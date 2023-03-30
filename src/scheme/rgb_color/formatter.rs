use crate::{
    scheme::rgb_color::RgbColor,
    template::color_field::{Dec, Format, Hex, Rgb},
};
use ramhorns::{encoding::Encoder, Content};

#[derive(Debug, Clone, Copy)]
pub struct RgbColorFormatter {
    pub color: RgbColor,
    pub format: Format,
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
            Format::Dec(Dec::B) => encoder.format_escaped(format_args!("{:.2}", g as f64 / 255.0)),
        }
    }
}
