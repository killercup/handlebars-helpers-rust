macro_rules! json_value_as {
    ($x:ident, object) => { $x.as_object() };
    ($x:ident, array) => { $x.as_array() };
    ($x:ident, string) => { $x.as_string() };
    ($x:ident, str) => { $x.as_str() };
    ($x:ident, i64) => { $x.as_i64() };
    ($x:ident, u64) => { $x.as_u64() };
    ($x:ident, f64) => { $x.as_f64() };
    ($x:ident, bool) => { $x.as_bool() };
    ($x:ident, null) => { $x.as_null() };
}

#[macro_export]
macro_rules! handlebars_helper {
    ($fn_name:ident: |$($name:ident: $tpe:tt),*| $body:expr ) => {
        #[allow(unused_assignments)]
        pub fn $fn_name(
            h: &handlebars::Helper,
            _: &handlebars::Handlebars,
            rc: &mut handlebars::RenderContext
        ) -> Result<(), handlebars::RenderError> {
            let mut param_idx = 0;

            $(
                let $name = h.param(param_idx)
                    .map(|x| x.value())
                    .ok_or_else(|| handlebars::RenderError::new(&format!(
                        "`{}` helper: Couldn't read parameter {}",
                        stringify!($fn_name), stringify!($name),
                    )))
                    .and_then(|x|
                        json_value_as!(x, $tpe)
                        .ok_or_else(|| handlebars::RenderError::new(&format!(
                            "`{}` helper: Couldn't convert parameter {} to type `{}`. \
                            It's {:?} as JSON. Got these params: {:?}",
                            stringify!($fn_name), stringify!($name), stringify!($tpe),
                            x, h.params(),
                        )))
                    )?;
                param_idx += 1;
            )*

            let result = $body;

            println!("debug!! {}({}): {}", stringify!($fn_name), concat!($(stringify!($name), ", "),*), $crate::json_to_string(&result).unwrap());

            // rc.writer().write($crate::json_to_string(&result)?.as_bytes())?;
            $crate::json_to_writer(rc.writer(), &result)?;
            Ok(())
        }
    };
}
