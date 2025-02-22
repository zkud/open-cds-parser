use super::IdentifierTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct FieldTerm {
    #[subnode_prop]
    name: Box<IdentifierTerm>,
    #[subnode_prop]
    type_name: Box<IdentifierTerm>,
}
