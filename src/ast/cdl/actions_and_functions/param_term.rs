use crate::ast::Location;

use super::super::super::ColonTerm;
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
    colon: Box<ColonTerm>,
    #[subnode_prop]
    type_reference: Box<TypeReferenceTerm>,
}
