use crate::{
    scheme::rgb_color::RgbColor,
    template::color_field::{Format, OutputFormat, OutputVariant},
};
use ramhorns::{encoding::Encoder, Content};

#[derive(Debug, Clone, Copy)]
pub struct RgbColorFormatter {
    pub color: RgbColor,
    pub format: Format,
}

impl RgbColorFormatter {
    pub fn is_supported_format(&self) -> bool {
        use OutputFormat::{Dec, Hex, Rgb};
        use OutputVariant::{B, G, R};

        matches!(
            self.format,
            Format {
                output_format: Hex,
                output_variant: OutputVariant::Rgb | OutputVariant::Bgr
            } | Format {
                output_format: Hex | Rgb | Dec,
                output_variant: R | G | B
            }
        )
    }
}

impl Content for RgbColorFormatter {
    fn is_truthy(&self) -> bool {
        self.is_supported_format()
    }

    fn render_escaped<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        use OutputFormat::{Dec, Hex, Rgb};
        let RgbColor([r, g, b]) = self.color;

        match self.format {
            Format {
                output_format: Hex,
                output_variant: OutputVariant::Rgb,
            } => encoder.format_escaped(format_args!("{r:02x}{g:02x}{b:02x}")),
            Format {
                output_format: Hex,
                output_variant: OutputVariant::R,
            } => encoder.format_escaped(format_args!("{r:02x}")),
            Format {
                output_format: Hex,
                output_variant: OutputVariant::G,
            } => encoder.format_escaped(format_args!("{g:02x}")),
            Format {
                output_format: Hex,
                output_variant: OutputVariant::B,
            } => encoder.format_escaped(format_args!("{b:02x}")),
            Format {
                output_format: Hex,
                output_variant: OutputVariant::Bgr,
            } => encoder.format_escaped(format_args!("{b:02x}{g:02x}{r:02x}")),

            Format {
                output_format: Rgb,
                output_variant: OutputVariant::R,
            } => encoder.format_escaped(format_args!("{r}")),
            Format {
                output_format: Rgb,
                output_variant: OutputVariant::G,
            } => encoder.format_escaped(format_args!("{g}")),
            Format {
                output_format: Rgb,
                output_variant: OutputVariant::B,
            } => encoder.format_escaped(format_args!("{b}")),

            Format {
                output_format: Dec,
                output_variant: OutputVariant::R,
            } => encoder.format_escaped(format_args!("{:.2}", r as f64 / 255.0)),
            Format {
                output_format: Dec,
                output_variant: OutputVariant::G,
            } => encoder.format_escaped(format_args!("{:.2}", g as f64 / 255.0)),
            Format {
                output_format: Dec,
                output_variant: OutputVariant::B,
            } => encoder.format_escaped(format_args!("{:.2}", g as f64 / 255.0)),
            Format {
                output_format: Rgb | Dec,
                output_variant: OutputVariant::Rgb | OutputVariant::Bgr,
            } => Ok(()),
        }
    }
}