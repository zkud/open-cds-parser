use proc_macro::TokenStream;
use proc_macro2::TokenStream as QuoteTokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

mod getters_setters;
mod new;
mod util;
mod visitor;

use getters_setters::impl_getters_and_setters_for_fields;
use new::impl_default_new_methods;
use visitor::impl_accept_visitor_method;

#[proc_macro_derive(ASTTerm, attributes(ast_term, prop, subnode_prop))]
pub fn ast_term(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input);
    let gen = impl_ast_term(&ast);
    gen.into()
}

fn impl_ast_term(input: &DeriveInput) -> QuoteTokenStream {
    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let mut tokens = impl_accept_visitor_method(input, fields);
            tokens.extend(impl_getters_and_setters_for_fields(input, fields));
            tokens.extend(impl_default_new_methods(input, fields));
            tokens
        }
        _ => quote! {},
    }
}
