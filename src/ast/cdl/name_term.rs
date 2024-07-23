use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug)]
#[ast_term(visitor_path = "process_name")]
pub struct NameTerm {
    #[prop]
    value: String,
}
