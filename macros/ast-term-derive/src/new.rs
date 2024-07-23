use proc_macro2::TokenStream as QuoteTokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Fields};

use super::util::type2str;

pub fn impl_default_new_methods(input: &DeriveInput, fields: &Fields) -> QuoteTokenStream {
    let name = &input.ident;
    let header = format!(" Creates a new instance of the {}.", name);
    let doc_params = get_doc_params(fields);
    let returns = format!(" * `{}` - A new instance of the {}", name, name);
    let params = get_new_params(fields);
    let struct_fields = get_new_struct_params(fields);
    quote! {
      impl #name {
        #[doc=#header]
        #[doc=""]
        #[doc=" # Arguments"]
        #[doc=""]
        #doc_params
        #[doc=""]
        #[doc=" # Returns"]
        #[doc=""]
        #[doc=#returns]
        #[cfg(not(tarpaulin_include))]
        pub fn new(#params) -> #name {
          #name {
            #struct_fields
          }
        }
      }
    }
}

fn get_doc_params(fields: &Fields) -> QuoteTokenStream {
    let mut tokens = quote! {};
    for field in fields.iter() {
        let field_tokens = get_doc_param_def(field);
        tokens.extend(field_tokens);
    }
    tokens
}

fn get_doc_param_def(field: &Field) -> QuoteTokenStream {
    if let Some(field_name) = &field.ident {
        let field_type = &field.ty;
        let value_arg = format!(
            " * `{}` - A value for the {} field",
            type2str(field_type),
            field_name
        );
        quote! {
          #[doc=#value_arg]
        }
    } else {
        quote! {}
    }
}

fn get_new_params(fields: &Fields) -> QuoteTokenStream {
    let mut tokens = quote! {};
    for field in fields.iter() {
        let field_tokens = get_param_def(field);
        tokens.extend(field_tokens);
    }
    tokens
}

fn get_param_def(field: &Field) -> QuoteTokenStream {
    if let Some(field_name) = &field.ident {
        let field_type = &field.ty;
        quote! {
          #field_name: #field_type,
        }
    } else {
        quote! {}
    }
}

fn get_new_struct_params(fields: &Fields) -> QuoteTokenStream {
    let mut tokens = quote! {};
    for field in fields.iter() {
        let field_tokens = get_struct_param_def(field);
        tokens.extend(field_tokens);
    }
    tokens
}

fn get_struct_param_def(field: &Field) -> QuoteTokenStream {
    if let Some(field_name) = &field.ident {
        quote! {
          #field_name,
        }
    } else {
        quote! {}
    }
}
