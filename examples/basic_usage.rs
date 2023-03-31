use base16_color_scheme::{Scheme, Template};
use std::fs::read_to_string;

fn main() {
    let template_str = read_to_string("examples/template.mustache").unwrap();
    let scheme_str = read_to_string("examples/scheme.yml").unwrap();

    let template = Template::new(template_str).unwrap();
    let mut scheme: Scheme = serde_yaml::from_str(&scheme_str).unwrap();
    scheme = scheme.create_slug();

    println!("{}", template.render(&scheme));
}
