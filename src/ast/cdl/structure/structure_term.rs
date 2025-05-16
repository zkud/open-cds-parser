use super::FieldTerm;
use crate::ast::PunctuationSignTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct StructureTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    open_brace: Box<PunctuationSignTerm>,
    #[subnode_prop]
    fields: Vec<FieldTerm>,
    #[subnode_prop]
    close_brace: Box<PunctuationSignTerm>,
}
