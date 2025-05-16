use super::super::IdentifierTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct FieldTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    name: Box<IdentifierTerm>,
    #[subnode_prop]
    type_name: Box<IdentifierTerm>,
}
