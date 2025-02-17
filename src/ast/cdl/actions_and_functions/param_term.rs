use crate::ast::Location;

use super::super::super::ColonTerm;
use super::super::NameTerm;
use super::super::TypeReferenceTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ParamTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    colon: Box<ColonTerm>,
    #[subnode_prop]
    type_reference: Box<TypeReferenceTerm>,
}
