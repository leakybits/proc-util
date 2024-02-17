use std::iter::once;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::*;
use syn::*;

/// Renders a macro expansion by parsing the `syn` input.
#[macro_export]
macro_rules! render {
($($args:ident,)* {$expr:expr}) => {{
    use $crate::*;

    $(
        let $args = match ::syn::parse($args) {
            Ok(args) => args,
            Err(e) => return e.to_compile_error().into(),
        };
    )*

    $expr.into_token_stream().into()
}};
}

/// Converts something into a `TokenStream`.
pub trait IntoTokenStream {
    fn into_token_stream(self) -> TokenStream;
}

/// A token stream can be converted into itself.
impl IntoTokenStream for TokenStream {
    fn into_token_stream(self) -> TokenStream {
        self
    }
}

/// A result can be converted into a `TokenStream`:
/// - If it's `Ok`, the inner `TokenStream` is returned.
/// - If it's `Err`, the error is converted into a compile error.
impl IntoTokenStream for Result<TokenStream> {
    fn into_token_stream(self) -> TokenStream {
        self.unwrap_or_else(|err| err.into_compile_error())
    }
}

/// A custom `Ident` which renders itself in a particular case.
#[derive(Debug)]
pub struct CaseIdent {
    ident: Ident,
    case: Case,
}

#[derive(Debug)]
enum Case {
    Snake,
    Camel,
}

impl CaseIdent {
    /// Creates a new snake-case `Ident`.
    pub fn snake(ident: Ident) -> Self {
        Self::new(ident, Case::Snake)
    }

    /// Creates a new camel-case `Ident`.
    pub fn camel(ident: Ident) -> Self {
        Self::new(ident, Case::Camel)
    }

    fn new(ident: Ident, case: Case) -> Self {
        CaseIdent { ident, case }
    }
}

impl ToTokens for CaseIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self.case {
            Case::Snake => self.ident.to_string().to_snake(),
            Case::Camel => self.ident.to_string().to_camel(),
        };

        format_ident!("{ident}").to_tokens(tokens);
    }
}

/// Extends the `str` type with some useful methods.
trait StringExt {
    /// Returns the first character of the string.
    fn head(&self) -> String;

    /// Converts the string to snake case.
    fn to_snake(&self) -> String;

    /// Converts the string to camel case.
    fn to_camel(&self) -> String;

    /// Appends the given extension to the string.
    fn extended(&self, ext: impl IntoIterator<Item = char>) -> String;
}

impl StringExt for str {
    fn head(&self) -> String {
        self.chars().take(1).collect()
    }

    fn to_snake(&self) -> String {
        self.chars()
            .tuple_windows()
            .fold(self.head().to_lowercase(), |res, (lhs, rhs)| {
                if lhs.is_lowercase() && rhs.is_uppercase() {
                    res.extended(once('_')).extended(rhs.to_lowercase())
                } else {
                    res.extended(rhs.to_lowercase())
                }
            })
    }

    fn to_camel(&self) -> String {
        self.chars()
            .tuple_windows()
            .fold(self.head().to_lowercase(), |res, (lhs, rhs)| {
                if rhs == '_' {
                    res
                } else if lhs == '_' {
                    res.extended(rhs.to_uppercase())
                } else {
                    res.extended(once(rhs))
                }
            })
    }

    fn extended(&self, ext: impl IntoIterator<Item = char>) -> String {
        let mut res = self.to_string();
        res.extend(ext);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake() {
        let case = [
            // --- empty ---
            ("", ""),
            // --- single character ---
            ("a", "a"),
            ("A", "a"),
            ("_", "_"),
            // --- all lowercase ---
            ("foo", "foo"),
            ("föö", "föö"),
            ("foobar", "foobar"),
            ("fööbar", "fööbar"),
            // --- already snake case ---
            ("foo_bar", "foo_bar"),
            ("föö_bar", "föö_bar"),
            ("foo_bar_baz", "foo_bar_baz"),
            ("föö_bar_baz", "föö_bar_baz"),
            // --- camel case ---
            ("fooBar", "foo_bar"),
            ("fööBär", "föö_bär"),
            ("fooBarBaz", "foo_bar_baz"),
            ("fööBärBaz", "föö_bär_baz"),
            // --- pascal case ---
            ("FooBar", "foo_bar"),
            ("FööBär", "föö_bär"),
            ("FooBarBaz", "foo_bar_baz"),
            ("FööBärBaz", "föö_bär_baz"),
        ];

        for (input, expected) in &case {
            assert_eq!(&input.to_snake(), expected);
        }
    }

    #[test]
    fn test_to_camel() {
        let case = [
            // // --- empty ---
            ("", ""),
            // // --- single character ---
            ("a", "a"),
            ("A", "a"),
            ("_", "_"),
            // // --- all lowercase ---
            ("foo", "foo"),
            ("föö", "föö"),
            ("foobar", "foobar"),
            ("fööbar", "fööbar"),
            // // --- already camel case ---
            ("fooBar", "fooBar"),
            ("fööBär", "fööBär"),
            ("fooBarBaz", "fooBarBaz"),
            ("fööBärBaz", "fööBärBaz"),
            // // --- snake case ---
            ("foo_bar", "fooBar"),
            ("föö_bar", "fööBar"),
            ("foo_bar_baz", "fooBarBaz"),
            ("föö_bar_baz", "fööBarBaz"),
            // // --- pascal case ---
            ("FooBar", "fooBar"),
            ("FööBär", "fööBär"),
            ("FooBarBaz", "fooBarBaz"),
            ("FööBärBaz", "fööBärBaz"),
        ];

        for (input, expected) in &case {
            assert_eq!(&input.to_camel(), expected);
        }
    }
}
