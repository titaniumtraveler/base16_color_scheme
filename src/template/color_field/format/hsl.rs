use crate::scheme::RgbColor;
use serde::{Deserialize, Serialize};

/// this implements rgb to hls conversion
///
/// see [`RgbFormatter`](crate::scheme::RgbColorFormatter) for details
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct HslFormatter {
    pub hue: f64,
    pub luminance: f64,
    pub saturation: f64,
}

impl HslFormatter {
    pub fn from_color(RgbColor([r, g, b]): RgbColor) -> Self {
        use std::cmp::Ordering::{Equal, Greater, Less};

        let color = [r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0];
        let min = color.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let (max_index, max) = color
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap();

        let luminance = (max + min) / 2.0;
        let saturation = match (min == max, luminance.total_cmp(&0.5)) {
            (true, _) => {
                return Self {
                    hue: 0.0,
                    luminance,
                    saturation: 0.0,
                };
            }
            (false, Less | Equal) => (max - min) / (max + min),
            (false, Greater) => (max - min) / (2.0 - max - min),
        };

        let [r, g, b] = color;
        let hue = match max_index {
            // Red is max
            0 => (g - b) / (max - min),
            // green is max
            1 => 2.0 + (b - r) / (max - min),
            // blue is max
            2 => 4.0 + (r - g) / (max - min),
            _ => unreachable!(),
        } * 60.0;

        Self {
            hue: hue.rem_euclid(360.0),
            luminance,
            saturation,
        }
    }
}
