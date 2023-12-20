use color_field::ColorField;
use serde::{Deserialize, Serialize};

pub mod color_field;

/// type representing a field in the mustache template
///
/// see <https://github.com/tinted-theming/home/main/builder.md#template-variables>
///
/// # Examples
///
/// ```rust
/// use base16_color_scheme::template::{
///     color_field::{ColorField, Format, Hex},
///     TemplateField,
/// };
///
/// assert_eq!(
///     TemplateField::parse_field("scheme-name"),
///     TemplateField::SchemeName
/// );
/// assert_eq!(
///     TemplateField::parse_field("scheme-author"),
///     TemplateField::SchemeAuthor
/// );
/// assert_eq!(
///     TemplateField::parse_field("scheme-slug"),
///     TemplateField::SchemeSlug
/// );
/// assert_eq!(
///     TemplateField::parse_field("base07-hex-r"),
///     TemplateField::ColorField(ColorField {
///         number: 0x07,
///         format: Format::Hex(Hex::R)
///     })
/// )
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TemplateField {
    /// this variant is returned for `scheme-name` and `scheme`
    SchemeName,
    /// this variant is returned for `scheme-author`
    SchemeAuthor,
    /// this variant is returned for `scheme-slug`
    SchemeSlug,
    /// this variant is returned for a color description
    ///
    /// see [`ColorField`](color_field::ColorField) and [`Format`](color_field::Format) for details
    ColorField(ColorField),
    /// this variant is returned if the parsed string is invalid
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
