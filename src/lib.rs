extern crate handlebars;
extern crate serde_json;

use handlebars::Handlebars;

#[macro_use]
mod macros;

mod helpers {
    use handlebars;
    use serde_json;

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
