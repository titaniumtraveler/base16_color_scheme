use hex::FromHexError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map_res,
    sequence::tuple,
    Finish, IResult, Parser,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub use self::format::{hsl::HslFormatter, Dec, Format, Hex, Hsl, Rgb};

mod format;

/// this represents a field in the template containing a color description defined by the
/// [specification](https://github.com/tinted-themiung/home/blob/main/builder.md#template-variables)
///
/// Note that in contrast to the base16 spec this supports up to 256 colors. \
/// (exactly what fits into a [`u8`])
///
/// This is used in combination with the [`Scheme`](crate::Scheme) to look up the specified color.\
/// See [`Scheme::color()`](crate::Scheme::color)
///
/// # Examples
///
/// ```rust
/// use base16_color_scheme::{
///     scheme::{BaseIndex, RgbColor, RgbColorFormatter},
///     template::color_field::{ColorField, Format, Hex},
///     Scheme,
/// };
/// use std::collections::BTreeMap;
///
/// let color_field: ColorField = "base07-hex-r".parse().unwrap();
/// let scheme = Scheme {
///     colors: BTreeMap::from([(BaseIndex(0x07), RgbColor([0x7c, 0xaf, 0xc2]))]),
///     ..Default::default()
/// };
///
/// assert_eq!(
///     scheme.color(color_field),
///     Some(RgbColorFormatter {
///         color: RgbColor([0x7c, 0xaf, 0xc2]),
///         format: Format::Hex(Hex::R)
///     })
/// )
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ColorField {
    pub number: u8,
    pub format: Format,
}

impl FromStr for ColorField {
    type Err = ColorFieldError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (input, (number, format)) = parse_field(input).finish().map_err(|_| ColorFieldError)?;

        if !input.is_empty() {
            return Err(ColorFieldError);
        }

        Ok(Self { number, format })
    }
}

fn parse_field(input: &str) -> IResult<&str, (u8, Format)> {
    let (input, (_, number, _, format)) = tuple((
        tag("base"),
        map_res(take(2usize), |input| -> Result<u8, FromHexError> {
            let mut hex = [0u8];
            hex::decode_to_slice(input, &mut hex)?;
            let [number] = hex;

            Ok(number)
        }),
        tag("-"),
        alt((
            tag("hex")
                .and(alt((
                    tag("-bgr").map(|_| Hex::Bgr),
                    tag("-r").map(|_| Hex::R),
                    tag("-g").map(|_| Hex::G),
                    tag("-b").map(|_| Hex::B),
                    tag("").map(|_| Hex::Rgb),
                )))
                .map(|(_, hex)| Format::Hex(hex)),
            tag("rgb")
                .and(alt((
                    tag("-r").map(|_| Rgb::R),
                    tag("-g").map(|_| Rgb::G),
                    tag("-b").map(|_| Rgb::B),
                )))
                .map(|(_, rgb)| Format::Rgb(rgb)),
            tag("dec")
                .and(alt((
                    tag("-r").map(|_| Dec::R),
                    tag("-g").map(|_| Dec::G),
                    tag("-b").map(|_| Dec::B),
                )))
                .map(|(_, dec)| Format::Dec(dec)),
            tag("hsl")
                .and(alt((
                    tag("-h").map(|_| Hsl::H),
                    tag("-s").map(|_| Hsl::S),
                    tag("-l").map(|_| Hsl::L),
                )))
                .map(|(_, hsl)| Format::Hsl(hsl)),
        )),
    ))(input)?;
    Ok((input, (number, format)))
}

/// Error returned by [`ColorField::from_str`]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColorFieldError;
