use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_path")]
pub struct PathTerm {
    #[prop]
    value: String,
}
