use super::super::super::Location;
use super::super::IdentifierTerm;
use super::TypeReferenceTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct TypeDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    resolved_type: Box<TypeReferenceTerm>,
}
