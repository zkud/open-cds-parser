use crate::ast::Location;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct SubIdentifierTerm {
    #[prop]
    location: Location,
    #[prop]
    value: String,
}
