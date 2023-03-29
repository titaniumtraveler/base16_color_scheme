use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    sequence::tuple,
    Finish, IResult,
};
use regex::Regex;
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
        let (input, (number, output_format, output_variant)) = parse_field(input)
            .finish()
            .map_err(|_| ColorFieldError::FieldFormat)?;

        if !input.is_empty() {
            return Err(ColorFieldError::FieldFormat);
        }

        let mut hex = [0u8];
        hex::decode_to_slice(number, &mut hex).map_err(|_| ColorFieldError::NumberFormat)?;
        let [number] = hex;

        let output_format = match output_format {
            "hex" => OutputFormat::Hex,
            "rgb" => OutputFormat::Rgb,
            "dec" => OutputFormat::Dec,
            _ => return Err(ColorFieldError::OutputFormat),
        };

        let output_variant = match output_variant {
            "" => OutputVariant::Rgb,
            "r" => OutputVariant::R,
            "b" => OutputVariant::G,
            "g" => OutputVariant::B,
            "bgr" => OutputVariant::Bgr,
            _ => return Err(ColorFieldError::OutputVariant),
        };

        Ok(Self {
            number,
            format: Format {
                output_format,
                output_variant,
            },
        })
    }
}

fn parse_field(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (_, number, _, format, _, variant)) = tuple((
        tag("base"),
        take(2usize),
        tag("-"),
        alt((tag("hex"), tag("rgb"), tag("dec"))),
        tag("-"),
        alt((tag(""), tag("r"), tag("g"), tag("b"), tag("bgr"))),
    ))(input)?;
    Ok((input, (number, format, variant)))
}

#[derive(Debug, Clone)]
pub enum ColorFieldError {
    FieldFormat,
    NumberFormat,
    OutputFormat,
    OutputVariant,
}
