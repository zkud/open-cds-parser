use proc_macro2::TokenStream as QuoteTokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Field, Fields};

use super::util::type2str;

pub fn impl_getters_and_setters_for_fields(
  input: &DeriveInput,
  fields: &Fields,
) -> QuoteTokenStream {
  let mut tokens = quote! {};
  for field in fields.iter() {
    let field_tokens = impl_getters_and_setters_for_field(input, field);
    tokens.extend(field_tokens);
  }
  tokens
}

fn impl_getters_and_setters_for_field(input: &DeriveInput, field: &Field) -> QuoteTokenStream {
  if !getters_setters_are_required(field) {
    return quote! {};
  }
  let mut tokens = quote! {};
  let getter = impl_getter(input, field);
  tokens.extend(getter);
  let getter = impl_mut_getter(input, field);
  tokens.extend(getter);
  let setter = impl_setter(input, field);
  tokens.extend(setter);
  tokens
}

fn getters_setters_are_required(field: &Field) -> bool {
  matches!(
    field
      .attrs
      .iter()
      .find(|attr| attr.path.is_ident("prop") || attr.path.is_ident("subnode_prop")),
    Some(_)
  )
}

fn impl_getter(input: &DeriveInput, field: &Field) -> QuoteTokenStream {
  if let Some(field_name) = &field.ident {
    let name = &input.ident;
    let field_type = &field.ty;
    let header = format!(
      " Returns a reference to the {} field of the {}.",
      field_name, name
    );
    let self_arg = format!(" * `&self` - A reference to the {}", name);
    let returns = format!(
      " * `&{}` - A reference to the {} field of the struct",
      type2str(field_type),
      field_name
    );
    quote! {
      impl #name {
        #[doc=#header]
        #[doc=""]
        #[doc=" # Arguments"]
        #[doc=""]
        #[doc=#self_arg]
        #[doc=""]
        #[doc=" # Returns"]
        #[doc=""]
        #[doc=#returns]
        #[cfg(not(tarpaulin_include))]
        pub fn #field_name(&self) -> &#field_type {
          &self.#field_name
        }
      }
    }
  } else {
    quote! {}
  }
}

fn impl_mut_getter(input: &DeriveInput, field: &Field) -> QuoteTokenStream {
  if let Some(field_name) = &field.ident {
    let method_name = format_ident!("{}_mut", field_name);
    let name = &input.ident;
    let field_type = &field.ty;
    let header = format!(
      " Returns a mutable reference to the {} field of the {}.",
      field_name, name
    );
    let self_arg = format!(" * `&mut self` - A mutable reference to the {}", name);
    let returns = format!(
      " * `&mut {}` - A mutable reference to the {} field of the struct",
      type2str(field_type),
      field_name
    );
    quote! {
      impl #name {
        #[doc=#header]
        #[doc=""]
        #[doc=" # Arguments"]
        #[doc=""]
        #[doc=#self_arg]
        #[doc=""]
        #[doc=" # Returns"]
        #[doc=""]
        #[doc=#returns]
        #[cfg(not(tarpaulin_include))]
        pub fn #method_name(&mut self) -> &mut #field_type {
          &mut self.#field_name
        }
      }
    }
  } else {
    quote! {}
  }
}

fn impl_setter(input: &DeriveInput, field: &Field) -> QuoteTokenStream {
  if let Some(field_name) = &field.ident {
    let method_name = format_ident!("set_{}", field_name);
    let name = &input.ident;
    let field_type = &field.ty;
    let header = format!(" Sets the {} field of the {}.", field_name, name);
    let self_arg = format!(" * `&mut self` - A mutable reference to the {}", name);
    let value_arg = format!(
      " * `{}` - A new value for the {} field",
      type2str(field_type),
      field_name
    );
    quote! {
      impl #name {
        #[doc=#header]
        #[doc=""]
        #[doc=" # Arguments"]
        #[doc=""]
        #[doc=#self_arg]
        #[doc=#value_arg]
        #[cfg(not(tarpaulin_include))]
        pub fn #method_name(&mut self, value: #field_type) {
          self.#field_name = value;
        }
      }
    }
  } else {
    quote! {}
  }
}
