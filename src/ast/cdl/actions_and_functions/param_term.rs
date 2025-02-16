use super::super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ParamTerm {
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    type_name: Box<NameTerm>,
}
