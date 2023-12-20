use crate::{
    scheme::rgb_color::RgbColor,
    template::color_field::{Dec, Format, Hex, Hsl, HslFormatter, Rgb},
};
use ramhorns::{encoding::Encoder, Content};
use std::fmt::{self, Display, Formatter};

/// formatter that formats a color according to the [specification](https://github.com/tinted-theming/home/blob/main/builder.md#template-variables)
///
/// This formatter contains a color and a specifier in what format this color should be formatted.
///
/// # Example
///
/// ```rust
/// # macro_rules! rgb_formatter_eq {
/// #     ($color:expr, $format:expr, $expected:expr) => {
/// #         assert_eq!(
/// #             format!(
/// #                 "{}",
/// #                 RgbColorFormatter {
/// #                     color: $color,
/// #                     format: $format
/// #                 }
/// #             ),
/// #             $expected
/// #         );
/// #     };
/// #     ($color:expr, $($format:expr, $expected:expr);*;) => {
/// #         $(rgb_formatter_eq!($color, $format, $expected);)*
/// #     };
/// # }
/// #
/// use base16_color_scheme::{
///     scheme::{RgbColor, RgbColorFormatter},
///     template::color_field::{Dec, Format, Hex, Hsl, Rgb},
/// };
///
/// let color = RgbColor([0x7c, 0xaf, 0xc2]);
///
/// // This macro is used to test the result of format!("{}") in a much more readable fashion
/// rgb_formatter_eq! {
///     color,
///     Format::Hex(Hex::Rgb), "7cafc2";
///     Format::Hex(Hex::R),   "7c";
///     Format::Hex(Hex::G),   "af";
///     Format::Hex(Hex::B),   "c2";
///     Format::Hex(Hex::Bgr), "c2af7c";
///     Format::Rgb(Rgb::R),   "124";
///     Format::Rgb(Rgb::G),   "175";
///     Format::Rgb(Rgb::B),   "194";
///     Format::Dec(Dec::R),   "0.49";
///     Format::Dec(Dec::G),   "0.69";
///     Format::Dec(Dec::B),   "0.76";
///     Format::Hsl(Hsl::H),   "196.29";
///     Format::Hsl(Hsl::S),   "0.36";
///     Format::Hsl(Hsl::L),   "0.62";
/// }
/// ```
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
