use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_type")]
pub struct TypeTerm {
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    resolved_type_name: Box<NameTerm>,
}
