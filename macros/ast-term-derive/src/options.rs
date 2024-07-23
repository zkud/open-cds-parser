use darling::FromDeriveInput;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(ast_term))]
pub struct Options {
    pub visitor_path: Option<String>,
}
