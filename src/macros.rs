/// Creates an `Ident` from an expression.
#[macro_export]
macro_rules! identify {
    ($expr:expr) => {{
        ::quote::format_ident!("_{}", $expr)
    }};
}

/// Renders a macro expansion by parsing the `syn` input.
#[macro_export]
macro_rules! render {
    ($($args:ident,)* {$expr:expr}) => {{
        use $crate::IntoTokenStream;

        $(
            let $args = match ::syn::parse($args) {
                Ok(args) => args,
                Err(e) => return e.to_compile_error().into(),
            };
        )*

        $expr.into_token_stream().into()
    }};
}
