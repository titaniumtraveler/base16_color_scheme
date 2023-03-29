use lazy_static::lazy_static;
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
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^base([0-9A-F]{2})-(hex|rgb|dec)(?:-(r|g|b|bgr))?$").unwrap();
        }

        let capture = RE.captures(input).ok_or(ColorFieldError::FieldFormat)?;
        let number = &capture[1];

        let mut hex = [0u8];
        hex::decode_to_slice(number, &mut hex).map_err(|_| ColorFieldError::NumberFormat)?;
        let [number] = hex;

        let output_format = match &capture[2] {
            "hex" => OutputFormat::Hex,
            "rgb" => OutputFormat::Rgb,
            "dec" => OutputFormat::Dec,
            _ => return Err(ColorFieldError::OutputFormat),
        };

        let output_variant = match capture.get(3).map(|m| m.as_str()) {
            Some("r") => OutputVariant::R,
            Some("b") => OutputVariant::G,
            Some("g") => OutputVariant::B,
            Some("bgr") => OutputVariant::Bgr,
            Some(_) => return Err(ColorFieldError::OutputVariant),
            None => OutputVariant::Rgb,
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

#[derive(Debug, Clone)]
pub enum ColorFieldError {
    FieldFormat,
    NumberFormat,
    OutputFormat,
    OutputVariant,
}
