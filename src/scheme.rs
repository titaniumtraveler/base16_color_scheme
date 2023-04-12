use crate::template::{color_field::ColorField, TemplateField};
use ramhorns::{encoding::Encoder, Content, Template};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub use self::{
    base_index::BaseIndex,
    rgb_color::{RgbColor, RgbColorFormatter},
};

mod base_index;
mod rgb_color;

/// type representing a base16 scheme
///
/// # Example
///
/// ```rust
/// use base16_color_scheme::{Scheme, Template};
///
/// let template = Template::new("\
/// {{scheme-name}} {{scheme-slug}} {{scheme-author}}
/// {{base00-hex}} {{base00-hex-bgr}}
/// {{base00-hex-r}} {{base00-hex-g}} {{base00-hex-b}}
/// {{base00-rgb-r}} {{base00-rgb-g}} {{base00-rgb-b}}
/// {{base00-dec-r}} {{base00-dec-g}} {{base00-dec-b}}
/// {{base00-hsl-h}} {{base00-hsl-s}} {{base00-hsl-l}}").unwrap();
///
/// let mut scheme: Scheme = serde_yaml::from_str(r#"
/// scheme: "Scheme Name"
/// author: "Scheme Author"
/// base00: "7cafc2"
/// "#).unwrap();
/// scheme = scheme.create_slug();
///
/// println!("{}", template.render(&scheme));
/// assert_eq!(
///     template.render(&scheme),
///     "\
/// Scheme Name scheme-name Scheme Author
/// 7cafc2 c2af7c
/// 7c af c2
/// 124 175 194
/// 0.49 0.69 0.76
/// 196.29 0.36 0.62");
/// ```
///
/// # Serialization / Deserialization
///
/// When deserializing Scheme requires the fields `scheme` and `author`,
/// ignores the field `scheme-slug`,
/// and accepts any number of fields between `base00` and `baseFF`, (any combination of uppercase and lowercase, so `baseab`, `baseAB`, and `baseAb` all work.)
/// though for a normal base16 theme it should have at least `base00` to `base0F`.
///
/// Because `scheme-slug` is not created while deserialization it has to be inserted manually.
/// Either by setting [`Scheme::slug`] manually, or using [`Scheme::create_slug`].
///
/// When serializing Scheme it first serializes [`Scheme::scheme`] and [`Scheme::author`] then ignores [`Scheme::slug`] as per [specification](https://github.com/chriskempson/base16/blob/main/file.md#scheme-files)
/// and afterwards serializes all colors contained in [`Scheme::colors`] ordered by the field number.\
/// (`base00`, `base01`, `base05` etc.)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scheme {
    pub scheme: String,
    pub author: String,
    #[serde(skip)]
    pub slug: String,
    #[serde(flatten)]
    pub colors: BTreeMap<BaseIndex, RgbColor>,
}

impl Scheme {
    pub fn scheme_name(&self) -> &str {
        &self.scheme
    }

    pub fn scheme_slug(&self) -> &str {
        &self.slug
    }

    /// Fill [`Self::slug`] based on [`Self::scheme`].
    ///
    /// see [`create_slug`]
    pub fn create_slug(mut self) -> Self {
        self.slug = create_slug(self.scheme_name());
        self
    }

    pub fn scheme_author(&self) -> &str {
        &self.author
    }

    /// Look up the field in [`Self::color`] and return a [formatter](RgbColorFormatter) for it.\
    /// (If the field exists.)
    pub fn color(&self, ColorField { number, format }: ColorField) -> Option<RgbColorFormatter> {
        self.colors
            .get(&BaseIndex(number))
            .map(|&color| RgbColorFormatter { color, format })
    }
}

/// create a slug from a scheme name based on the [specification](https://github.com/chriskempson/base16/blob/main/builder.md#template-tags).
///
/// # Example
///
/// ```rust
/// use base16_color_scheme::scheme::create_slug;
///
/// let name = "Scheme Name";
/// let slug = create_slug(name);
/// assert_eq!(slug, "scheme-name");
/// ```
pub fn create_slug(scheme_name: &str) -> String {
    scheme_name
        .chars()
        .flat_map(|c| match c {
            ' ' => '-'.to_lowercase(),
            _ => c.to_lowercase(),
        })
        .collect()
}

impl Content for Scheme {
    fn is_truthy(&self) -> bool {
        true
    }

    fn capacity_hint(&self, _tpl: &Template) -> usize {
        self.scheme_name().len()
            + self.scheme_author().len()
            + self.scheme_slug().len()
            + self.colors.len() * 6 // Amount of colors * ( 2 characters * 3 color components)
    }

    fn render_field_escaped<E: Encoder>(
        &self,
        _hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> Result<bool, E::Error> {
        use TemplateField::{ColorField, SchemeAuthor, SchemeName, SchemeSlug, UnparsableField};

        match TemplateField::parse_field(name) {
            SchemeName => encoder.write_escaped(self.scheme_name()).map(|_| true),
            SchemeAuthor => encoder.write_escaped(self.scheme_author()).map(|_| true),
            SchemeSlug => match self.scheme_slug() {
                "" => encoder.write_escaped("scheme-slug").map(|_| true),
                slug => encoder.write_escaped(slug).map(|_| true),
            },
            ColorField(color_field) => match self.color(color_field) {
                Some(value) => value.render_escaped(encoder).map(|_| true),
                None => Ok(false),
            },
            UnparsableField => Ok(false),
        }
    }

    fn render_field_inverse<C, E>(
        &self,
        _hash: u64,
        name: &str,
        section: ramhorns::Section<C>,
        encoder: &mut E,
    ) -> Result<bool, E::Error>
    where
        C: ramhorns::traits::ContentSequence,
        E: Encoder,
    {
        use TemplateField::{ColorField, SchemeAuthor, SchemeName, SchemeSlug, UnparsableField};

        match TemplateField::parse_field(name) {
            SchemeName => encoder.write_escaped(self.scheme_name()).map(|_| true),
            SchemeAuthor => encoder.write_escaped(self.scheme_author()).map(|_| true),
            SchemeSlug => match self.scheme_slug() {
                "" => encoder.write_escaped("scheme-slug").map(|_| true),
                slug => encoder.write_escaped(slug).map(|_| true),
            },
            ColorField(color_field) => match self.color(color_field) {
                Some(value) => value.render_inverse(section, encoder).map(|_| true),
                None => Ok(false),
            },
            UnparsableField => Ok(false),
        }
    }
}
