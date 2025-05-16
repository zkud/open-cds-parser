use super::super::Location;
use super::structure::StructureTerm;
use super::IdentifierTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct EntityTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    applied_aspects: Vec<IdentifierTerm>,
    #[subnode_prop]
    structure: Box<StructureTerm>,
}
