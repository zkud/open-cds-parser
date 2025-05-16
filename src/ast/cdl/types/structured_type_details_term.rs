use super::super::super::Location;
use super::super::structure::StructureTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct StructuredTypeDetailsTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    structure: Box<StructureTerm>,
}
