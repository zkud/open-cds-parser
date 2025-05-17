use super::super::TypeReferenceTerm;
use crate::ast::PunctuationSignTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct FieldTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    name: Box<IdentifierTerm>,
    #[subnode_prop]
    colon: Box<PunctuationSignTerm>,
    #[subnode_prop]
    type_name: Box<TypeReferenceTerm>,
    #[subnode_prop]
    semicolumn: Box<PunctuationSignTerm>,
}
