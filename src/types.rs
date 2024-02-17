use std::marker::PhantomData;

use proc_macro2::TokenStream;
use quote::*;
use syn::*;

use crate::case::{to_camel, to_snake};

/// A custom `Ident` which renders itself in a particular case.
#[derive(Debug)]
pub struct CaseIdent<C>(Ident, PhantomData<C>);

/// The snake-case variant.
struct Snake;

/// The camel-case variant.
struct Camel;

impl CaseIdent<Snake> {
    /// Creates a new snake-case `Ident`.
    pub fn snake(ident: Ident) -> Self {
        CaseIdent(ident, PhantomData)
    }
}

impl CaseIdent<Camel> {
    /// Creates a new camel-case `Ident`.
    pub fn camel(ident: Ident) -> Self {
        CaseIdent(ident, PhantomData)
    }
}

impl ToTokens for CaseIdent<Snake> {
    fn to_tokens(&self, output: &mut TokenStream) {
        format_ident!("{}", to_snake(&self.0)).to_tokens(output);
    }
}

impl ToTokens for CaseIdent<Camel> {
    fn to_tokens(&self, output: &mut TokenStream) {
        format_ident!("{}", to_camel(&self.0)).to_tokens(output);
    }
}
