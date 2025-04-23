use super::super::super::Location;
use ast_term_derive::ASTTerm;

// TODO: Reuse code from entity to define fields for structures
#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct StructuredTypeDetailsTerm {
    #[prop]
    location: Location,
}
