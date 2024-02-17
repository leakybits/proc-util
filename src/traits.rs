use proc_macro2::TokenStream;
use syn::Result;

/// Converts something into a `TokenStream`.
pub trait IntoTokenStream {
    /// Convert self into a `TokenStream`.
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
