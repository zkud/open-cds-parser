use super::field_term::FieldTerm;
use super::IdentifierTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct EntityTerm {
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    applied_aspects: Vec<IdentifierTerm>,
    #[subnode_prop]
    fields: Vec<FieldTerm>,
}
