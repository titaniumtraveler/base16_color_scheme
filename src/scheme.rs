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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn create_slug(mut self) -> Self {
        self.slug = create_slug(self.scheme_name());
        self
    }

    pub fn scheme_author(&self) -> &str {
        &self.author
    }

    pub fn color(&self, ColorField { number, format }: ColorField) -> Option<RgbColorFormatter> {
        self.colors
            .get(&BaseIndex(number))
            .map(|&color| RgbColorFormatter { color, format })
    }
}

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
