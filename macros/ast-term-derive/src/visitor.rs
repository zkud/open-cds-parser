use proc_macro2::TokenStream as QuoteTokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Fields};

pub fn impl_accept_visitor_method(
    input: &DeriveInput,
    fields: &Fields,
) -> QuoteTokenStream {
    let name = &input.ident;
    let fields_calls = impl_accept_visitor_method_for_fields(fields);
    quote! {
        use crate::ast::common::*;
        impl ASTTerm for #name {}

        impl Visitable for #name {
            #[cfg(not(tarpaulin_include))]
            fn accept<V: crate::visitor::Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
                visitor.process(self)?;
                #fields_calls
                Ok(())
            }
        }

        impl Convertable for #name {
            #[cfg(not(tarpaulin_include))]
            fn try_convert<T: Convertable>(&self) -> Option<T> {
                let self_any = self as &dyn std::any::Any;
                self_any.downcast_ref::<T>().cloned()
            }
        }
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
