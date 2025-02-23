use ast_term_derive::ASTTerm;

use super::super::{AsTerm, IdentifierTerm, ImportIdentifierTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct SelectorTerm {
    location: Location,
    import_identifier: Box<ImportIdentifierTerm>,
    as_term: Option<Box<AsTerm>>,
    alias: Option<Box<IdentifierTerm>>,
}
