# base16_color_scheme

A library to build [base16](https://github.com/chriskempson/base16) colorschemes written in Rust.

It uses [ramhorns](https://docs.rs/ramhorns/latest/ramhorns/index.html)
as template engine and therefore is fairly fast. \
(Around 70 ms - 200 ms for a 9 mb template I generated based on <https://github.com/chriskempson/base16-templates-source>.)

## Getting Started

To get started use you need to create a [`Template`](https://docs.rs/base16_color_scheme/0.3.1/base16_color_scheme/struct.Template.html) and a [`Scheme`](https://docs.rs/base16_color_scheme/0.3.1/base16_color_scheme/scheme/struct.Scheme.html).

A [`Template`](https://docs.rs/ramhorns/0.14.0/ramhorns/struct.Template.html) can be created by just reading the template file and using
[`Template::new()`](https://docs.rs/ramhorns/0.14.0/ramhorns/struct.Template.html#method.new).

A [`Scheme`](https://docs.rs/base16_color_scheme/0.3.1/base16_color_scheme/scheme/struct.Scheme.html) is often created by deserializing using [`serde`](https://docs.rs/serde/latest/serde).

Neither [`Template`](https://docs.rs/ramhorns/0.14.0/ramhorns/struct.Template.html) nor [`Scheme`](https://docs.rs/base16_color_scheme/0.3.1/base16_color_scheme/scheme/struct.Scheme.html) get modified by the rendering process,
which means both can be reused for efficiency.

```rust
use base16_color_scheme::{Scheme, Template};
use std::fs::read_to_string;

let template_str = read_to_string("path/to/template.mustache").unwrap();
let scheme_str = read_to_string("path/to/scheme.yml").unwrap();

let template = Template::new(template_str).unwrap();
let scheme: Scheme = serde_yaml::from_str(&scheme_str).unwrap();

template
    .render_to_file("path/to/rendered/template", &scheme)
    .unwrap();
```

## How it works

Internally the crate works by implementing [`ramhorns`](https://docs.rs/ramhorns/latest/ramhorns/index.html)'s [`Content`](https://docs.rs/ramhorns/0.14.0/ramhorns/trait.Content.html) trait.
When the rendering process tries to look up a field, the field name gets
parsed into a [`TemplateField`](https://docs.rs/base16_color_scheme/0.3.1/base16_color_scheme/template/enum.TemplateField.html). If it is a color, this color is fetched
from the [`Scheme`](https://docs.rs/base16_color_scheme/0.3.1/base16_color_scheme/scheme/struct.Scheme.html) and formatted as specified by <https://github.com/chriskempson/base16/blob/main/builder.md#template-tags>.
