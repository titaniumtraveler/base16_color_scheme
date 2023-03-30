# base16_color_scheme

A library to build [base16](https://github.com/chriskempson/base16) colorschemes written in Rust.

It uses [ramhorns](https://docs.rs/ramhorns/latest/ramhorns/index.html)
as template engine and therefore is fairly fast. \
(Around 70 ms - 200 ms for a 9 mb template I generated based on <https://github.com/chriskempson/base16-templates-source>.)

## Getting Started

To get started use you need to create a `Template` and a `Scheme`.

A `Template` can be created by just reading the template file and using
`Template::new()`.

A `Scheme` is often created by deserializing using `serde`.

Neither `Template` nor `Scheme` get modified by the rendering process,
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

Internally the crate works by implementing `ramhorns`'s `Content` trait.
When the rendering process tries to look up a field, the field name gets
parsed into a `TemplateField`. If it is a color, this color is fetched
from the `Scheme` and formatted as specified by <https://github.com/chriskempson/base16/blob/main/builder.md#template-tags>.
