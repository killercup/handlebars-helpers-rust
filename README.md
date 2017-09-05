# Handlebars Helpers Rust

[![Build Status](https://travis-ci.org/killercup/handlebars-helpers-rust.svg?branch=master)](https://travis-ci.org/killercup/handlebars-helpers-rust)

## What it does

It adds helpers to [handlebars-rust]:

[handlebars-rust]: https://github.com/sunng87/handlebars-rust

```rust
extern crate handlebars;
extern crate handlebars_helpers;

let mut renderer = handlebars::Handlebars::new();
handle_helpers::register(&mut renderer);
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
