use quote::quote;
use syn::Type;

pub fn type2str(rust_type: &Type) -> String {
  let rust_type = quote! {#rust_type};
  let rust_type = format!("{}", rust_type);
  rust_type.replace(" ", "")
}
