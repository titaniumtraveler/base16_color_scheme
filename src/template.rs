use color_field::ColorField;
use serde::{Deserialize, Serialize};

pub mod color_field;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TemplateField {
    SchemeName,
    SchemeAuthor,
    SchemeSlug,
    ColorField(ColorField),
    UnparsableField,
}

impl TemplateField {
    pub fn parse_field(name: &str) -> Self {
        match name {
            "scheme-name" | "scheme" => Self::SchemeName,
            "scheme-author" => Self::SchemeAuthor,
            "scheme-slug" => Self::SchemeSlug,
            _ => {
                if let Ok(field_spec) = name.parse() {
                    Self::ColorField(field_spec)
                } else {
                    Self::UnparsableField
                }
            }
        }
    }
}
