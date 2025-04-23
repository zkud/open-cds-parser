use ast_term_derive::ASTTerm;

use super::super::{AsTerm, IdentifierTerm, ImportIdentifierTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct SelectorTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    import_identifier: Box<ImportIdentifierTerm>,
    #[subnode_prop]
    as_term: Option<Box<AsTerm>>,
    #[subnode_prop]
    alias: Option<Box<IdentifierTerm>>,
}
