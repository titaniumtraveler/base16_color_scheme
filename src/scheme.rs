use crate::{
    scheme::rgb_color::RgbColorFormatter,
    template::{color_field::ColorField, TemplateField},
};
use ramhorns::{encoding::Encoder, Content, Template};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub use self::{base_index::BaseIndex, rgb_color::RgbColor};

mod base_index;
mod rgb_color;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scheme {
    pub scheme: String,
    pub author: String,
    #[serde(flatten)]
    pub colors: BTreeMap<BaseIndex, RgbColor>,
}

impl Scheme {
    pub fn scheme_name(&self) -> &str {
        &self.scheme
    }

    pub fn scheme_slug(&self) -> &str {
        &self.scheme
    }

    pub fn scheme_author(&self) -> &str {
        &self.author
    }

    pub fn color(&self, ColorField { number, format }: ColorField) -> Option<RgbColorFormatter> {
        if let Some(&color) = self.colors.get(&BaseIndex(number)) {
            Some(RgbColorFormatter { color, format })
        } else {
            None
        }
    }
}

impl Content for Scheme {
    fn is_truthy(&self) -> bool {
        true
    }

    fn capacity_hint(&self, _tpl: &Template) -> usize {
        // TODO
        0
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
            SchemeSlug => encoder.write_escaped(&self.scheme_slug()).map(|_| true),
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
            SchemeSlug => encoder.write_escaped(&self.scheme_slug()).map(|_| true),
            ColorField(field_spec) => match self.color(field_spec) {
                Some(value) => value.render_inverse(section, encoder).map(|_| true),
                None => Ok(false),
            },
            UnparsableField => Ok(false),
        }
    }
}
