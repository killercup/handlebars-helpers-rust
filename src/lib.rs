extern crate handlebars;
extern crate serde_json;

use handlebars::Handlebars;
use serde_json::{to_writer as json_to_writer, to_string as json_to_string};

#[macro_use] mod macros;

mod helpers {
    use handlebars;

    handlebars_helper!(gt: |x: u64, y: u64| x > y);
    handlebars_helper!(gte: |x: u64, y: u64| x >= y);
    handlebars_helper!(lt: |x: u64, y: u64| x < y);
    handlebars_helper!(lte: |x: u64, y: u64| x <= y);

    handlebars_helper!(not: |x: bool| !x);
}

pub fn register(handle: &mut Handlebars) {
    macro_rules! register {
        ($name:ident) => {
            handle.register_helper(stringify!($name), Box::new(helpers::$name));
        };
    }

    register!(gt);
    register!(gte);
    register!(lt);
    register!(lte);
    register!(not);
}
