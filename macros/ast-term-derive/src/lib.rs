use proc_macro;
use quote::quote;
use syn;

#[proc_macro_derive(ASTTerm, attributes(sub_term))]
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
  if !has_sub_node_attribute(field) {
    return proc_macro::TokenStream::new();
  }
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

fn has_sub_node_attribute(field: &syn::Field) -> bool {
  matches!(
    field
      .attrs
      .iter()
      .find(|attr| attr.path.is_ident("sub_term")),
    Some(_)
  )
}
