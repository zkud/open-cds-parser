use super::super::IdentifierTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct TypeDeclarationTerm {
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    resolved_type_name: Box<IdentifierTerm>,
}
