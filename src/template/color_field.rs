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

pub use self::format::{Format, OutputFormat, OutputVariant};

mod format;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    use OutputFormat::{Dec, Hex, Rgb};
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
                    tag("").map(|_| OutputVariant::Rgb),
                    tag("-r").map(|_| OutputVariant::R),
                    tag("-g").map(|_| OutputVariant::G),
                    tag("-b").map(|_| OutputVariant::B),
                    tag("-bgr").map(|_| OutputVariant::Bgr),
                )))
                .map(|(_, output_variant)| Format {
                    output_format: Hex,
                    output_variant,
                }),
            tag("rgb")
                .and(alt((
                    tag("-r").map(|_| OutputVariant::R),
                    tag("-g").map(|_| OutputVariant::G),
                    tag("-b").map(|_| OutputVariant::B),
                )))
                .map(|(_, output_variant)| Format {
                    output_format: Rgb,
                    output_variant,
                }),
            tag("dec")
                .and(alt((
                    tag("-r").map(|_| OutputVariant::R),
                    tag("-g").map(|_| OutputVariant::G),
                    tag("-b").map(|_| OutputVariant::B),
                )))
                .map(|(_, output_variant)| Format {
                    output_format: Dec,
                    output_variant,
                }),
        )),
    ))(input)?;
    Ok((input, (number, format)))
}

#[derive(Debug, Clone)]
pub struct ColorFieldError;
