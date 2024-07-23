use proc_macro2::TokenStream as QuoteTokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Field, Fields};

use super::options::Options;

pub fn impl_accept_visitor_method(
    input: &DeriveInput,
    fields: &Fields,
    options: &Options,
) -> QuoteTokenStream {
    if let Some(ref path) = options.visitor_path {
        let name = &input.ident;
        let method_name = format_ident!("{}", &path);
        let fields_calls = impl_accept_visitor_method_for_fields(fields);
        quote! {
          impl crate::ast::common::ast_term::ASTTerm for #name {
            #[cfg(not(tarpaulin_include))]
            fn accept(&self, visitor: &mut dyn crate::visitor::Visitor) -> Result<(), crate::visitor::VisitorError> {
              visitor.#method_name(self)?;
              #fields_calls
              Ok(())
            }
          }
        }
    } else {
        quote! {}
    }
}

fn impl_accept_visitor_method_for_fields(fields: &Fields) -> QuoteTokenStream {
    let mut tokens = quote! {};
    for field in fields.iter() {
        let field_tokens = impl_accept_visitor_method_for_field(field);
        tokens.extend(field_tokens);
    }
    tokens
}

fn impl_accept_visitor_method_for_field(field: &Field) -> QuoteTokenStream {
    if !field_accept_call_is_required(field) {
        return quote! {};
    }
    if let Some(field_name) = &field.ident {
        quote! {
          self.#field_name.accept(visitor)?;
        }
    } else {
        quote! {}
    }
}

fn field_accept_call_is_required(field: &Field) -> bool {
    matches!(
        field
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("subnode_prop")),
        Some(_)
    )
}
