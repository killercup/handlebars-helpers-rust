extern crate handlebars;
extern crate handlebars_helpers;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;

#[test]
fn nested_conditions() {
    let mut handlebars = Handlebars::new();
    handlebars_helpers::register(&mut handlebars);

    let result = handlebars
        .render_template("{{#if (gt 5 3)}}lorem{{else}}ipsum{{/if}}", &json!({}))
        .unwrap();
    assert_eq!(&result, "lorem");

    let result = handlebars
        .render_template(
            "{{#if (not (gt 5 3))}}lorem{{else}}ipsum{{/if}}",
            &json!({}),
        )
        .unwrap();
    assert_eq!(&result, "ipsum");
}
