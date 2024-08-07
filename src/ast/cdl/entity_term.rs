use super::field_term::FieldTerm;
use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct EntityTerm {
    #[subnode_prop]
    name: Box<NameTerm>,
    #[subnode_prop]
    applied_aspects: Vec<NameTerm>,
    #[subnode_prop]
    fields: Vec<FieldTerm>,
}
