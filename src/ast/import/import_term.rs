use super::super::common::Location;
use super::super::{KeywordTerm, PathTerm, SelectionBlockTerm, SemicolumnTerm};
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ImportTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    using: Box<KeywordTerm>,
    #[subnode_prop]
    selection_block: Box<SelectionBlockTerm>,
    #[subnode_prop]
    from: Box<KeywordTerm>,
    #[subnode_prop]
    path: Box<PathTerm>,
    #[subnode_prop]
    semicolumn: Box<SemicolumnTerm>,
}
