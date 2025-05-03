use super::super::super::KeywordTerm;
use super::super::super::SemicolumnTerm;
use super::super::IdentifierTerm;
use super::super::ParametersBlockTerm;
use super::super::ReturnsDeclarationTerm;
use crate::ast::Location;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct FunctionDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    function: Box<KeywordTerm>,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    parameters: Box<ParametersBlockTerm>,
    #[subnode_prop]
    returns: Box<ReturnsDeclarationTerm>,
    #[subnode_prop]
    semicolumn: Box<SemicolumnTerm>,
}
