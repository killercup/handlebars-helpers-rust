extern crate handlebars;
extern crate handlebars_helpers;
#[macro_use] extern crate serde_json;

use handlebars::Handlebars;

#[test]
fn nested_conditions() {
    let mut handlebars = Handlebars::new();
    handlebars_helpers::register(&mut handlebars);

    let result = handlebars.template_render(
        "{{#if (gt 5 3)}}lorem{{else}}ipsum{{/if}}", &json!({})
    ).unwrap();
    assert_eq!(&result, "lorem");

    // let result = handlebars.template_render(
    //     "{{#if (not (gt 5 3))}}lorem{{else}}ipsum{{/if}}", &json!({})
    // ).unwrap();
    // assert_eq!(&result, "ipsum");
}
