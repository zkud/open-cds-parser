use proc_macro;
use quote::quote;
use syn;
use proc_macro2::{Span};

#[proc_macro_derive(ASTTerm, attributes(prop))]
pub fn ast_term(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse_macro_input!(input);
  let gen = impl_ast_term(&ast);
  gen.into()
}

fn impl_ast_term(input: &syn::DeriveInput) -> proc_macro::TokenStream {
  match &input.data {
    syn::Data::Struct(syn::DataStruct { fields, .. }) => {
      let mut tokens = proc_macro::TokenStream::new();
      for field in fields.iter() {
        let field_tokens = impl_ast_term_for_field(input, field);
        tokens.extend(field_tokens);
      }
      tokens
    }
    _ => proc_macro::TokenStream::new(),
  }
}

fn impl_ast_term_for_field(
  input: &syn::DeriveInput,
  field: &syn::Field,
) -> proc_macro::TokenStream {
  if !is_property(field) {
    return proc_macro::TokenStream::new();
  }
  let mut tokens = proc_macro::TokenStream::new();
  let getter  = impl_getter(input, field);
  tokens.extend(getter);
  let getter  = impl_mut_getter(input, field);
  tokens.extend(getter);
  tokens
}

fn is_property(field: &syn::Field) -> bool {
  matches!(
    field
      .attrs
      .iter()
      .find(|attr| attr.path.is_ident("prop")),
    Some(_)
  )
}

fn impl_getter(input: &syn::DeriveInput, field: &syn::Field) -> proc_macro::TokenStream {
  if let Some(field_name) = &field.ident {
    let name = &input.ident;
    let field_type = &field.ty;
    quote! {
      impl #name {
        pub fn #field_name(&self) -> &#field_type {
          &self.#field_name
        }
      }
    }
    .into()
  } else {
    proc_macro::TokenStream::new()
  }
}

fn impl_mut_getter(input: &syn::DeriveInput, field: &syn::Field) -> proc_macro::TokenStream {
  if let Some(field_name) = &field.ident {
    let method_name = field_name.to_string() + "_mut";
    let method_name = syn::Ident::new(&method_name, Span::call_site());
    let name = &input.ident;
    let field_type = &field.ty;
    quote! {
      impl #name {
        pub fn #method_name(&mut self) -> &mut #field_type {
          &mut self.#field_name
        }
      }
    }
    .into()
  } else {
    proc_macro::TokenStream::new()
  }
}