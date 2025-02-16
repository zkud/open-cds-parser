use super::super::NameTerm;
use super::super::ParamTerm;
use super::super::ReturnsTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct FunctionTerm {
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    params: Vec<ParamTerm>,
    #[subnode_prop]
    returned_type: Box<ReturnsTerm>,
}
