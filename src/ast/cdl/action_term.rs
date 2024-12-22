use crate::ast::Location;

use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ActionTerm {
    location: Location,
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    params: Vec<ParamTerm>,
    #[subnode_prop]
    returned_type: Option<Box<ReturnsTerm>>,
}
