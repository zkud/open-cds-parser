use crate::ast::Location;

use super::super::super::PunctuationSignTerm;
use super::super::IdentifierTerm;
use super::super::TypeReferenceTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ParamTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    name: Box<IdentifierTerm>,
    #[subnode_prop]
    colon: Box<PunctuationSignTerm>,
    #[subnode_prop]
    type_reference: Box<TypeReferenceTerm>,
}
