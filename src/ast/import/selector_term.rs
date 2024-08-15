use ast_term_derive::ASTTerm;

use super::super::{AsTerm, ImportIdentifierTerm, NameTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct SelectorTerm {
    import_identifier: Box<ImportIdentifierTerm>,
    as_term: Option<Box<AsTerm>>,
    alias: Option<Box<NameTerm>>,
}
