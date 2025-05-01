use crate::ast::Location;

use super::super::super::SemicolumnTerm;
use super::super::IdentifierTerm;
use super::super::ReturnsDeclarationTerm;
use crate::ast::basic::KeywordTerm;
use super::ParametersBlockTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ActionDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    action: Box<KeywordTerm>,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    parameters: Box<ParametersBlockTerm>,
    #[subnode_prop]
    returns: Option<Box<ReturnsDeclarationTerm>>,
    #[subnode_prop]
    semicolumn: Box<SemicolumnTerm>,
}
