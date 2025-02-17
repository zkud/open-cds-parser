use crate::ast::Location;

use super::super::super::SemicolumnTerm;
use super::super::NameTerm;
use super::super::ReturnsDeclarationTerm;
use super::ActionTerm;
use super::ParametersBlockTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ActionDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    action: Box<ActionTerm>,
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    parameters: Box<ParametersBlockTerm>,
    #[subnode_prop]
    returned_type: Option<Box<ReturnsDeclarationTerm>>,
    #[subnode_prop]
    semicolumn: Box<SemicolumnTerm>,
}
