use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct TypeTerm {
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    resolved_type_name: Box<NameTerm>,
}
