use crate::ast::Location;

use super::super::super::CloseRoundBracketTerm;
use super::super::super::OpenRoundBracketTerm;
use super::super::super::SemicolumnTerm;
use super::super::NameTerm;
use super::super::ParamTerm;
use super::super::ReturnsTerm;
use super::ActionTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ActionDeclarationTerm {
    location: Location,
    #[subnode_prop]
    action: Box<ActionTerm>,
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    open_bracket: Box<OpenRoundBracketTerm>,
    #[subnode_prop]
    params: Vec<ParamTerm>,
    #[subnode_prop]
    close_bracket: Box<CloseRoundBracketTerm>,
    #[subnode_prop]
    returned_type: Option<Box<ReturnsTerm>>,
    #[subnode_prop]
    semicolumn: Box<SemicolumnTerm>,
}
