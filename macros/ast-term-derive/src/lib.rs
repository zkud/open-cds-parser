use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as QuoteTokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, Type};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(ast_term))]
struct Options {
  visitor_path: Option<String>,
}

#[proc_macro_derive(ASTTerm, attributes(ast_term, prop, subnode_prop))]
pub fn ast_term(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input);
  let opts = Options::from_derive_input(&ast).expect("Wrong options");
  let gen = impl_ast_term(&ast, opts);
  gen.into()
}

fn impl_ast_term(input: &DeriveInput, options: Options) -> QuoteTokenStream {
  match &input.data {
    Data::Struct(DataStruct { fields, .. }) => {
      let mut tokens = impl_accept_visitor_method(input, fields, &options);
      tokens.extend(impl_getters_and_setters_for_fields(input, fields));
      tokens.extend(impl_default_new_methods(input, fields, &options));
      tokens
    }
    _ => quote! {},
  }
}

fn impl_accept_visitor_method(
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

fn impl_getters_and_setters_for_fields(input: &DeriveInput, fields: &Fields) -> QuoteTokenStream {
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

fn impl_default_new_methods(
  input: &DeriveInput,
  fields: &Fields,
  options: &Options,
) -> QuoteTokenStream {
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

fn type2str(rust_type: &Type) -> String {
  let rust_type = quote! {#rust_type};
  let rust_type = format!("{}", rust_type);
  rust_type.replace(" ", "")
}
