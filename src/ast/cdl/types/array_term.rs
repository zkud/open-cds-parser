use super::super::super::Location;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ArrayTerm {
    #[prop]
    location: Location,
}
