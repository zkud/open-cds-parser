use ast_term_derive::ASTTerm;

use super::super::{IdentifierTerm, ImportIdentifierTerm, KeywordTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct SelectorTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    import_identifier: Box<ImportIdentifierTerm>,
    #[subnode_prop]
    as_term: Option<Box<KeywordTerm>>,
    #[subnode_prop]
    alias: Option<Box<IdentifierTerm>>,
}
