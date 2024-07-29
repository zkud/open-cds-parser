use ast_term_derive::ASTTerm;

use super::{alias_term::AliasTerm, import_identifier_term::ImportIdentifierTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_selector")]
pub struct SelectorTerm {
    import_identifier: Box<ImportIdentifierTerm>,
    alias: Option<Box<AliasTerm>>,
}
