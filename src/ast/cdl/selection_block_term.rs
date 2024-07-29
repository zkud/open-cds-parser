use super::super::super::visitor::{Visitor, VisitorError};
use super::super::common::ast_term::ASTTerm;
use super::{CloseFigureBracketTerm, CommaTerm, OpenFigureBracketTerm, SelectorTerm};
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_selection_block")]
pub struct SelectionBlockTerm {
    #[subnode_prop]
    open_brace: Option<Box<OpenFigureBracketTerm>>,
    #[subnode_prop]
    selectors: Vec<SelectionBlockSegment>,
    #[subnode_prop]
    close_brace: Option<Box<CloseFigureBracketTerm>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SelectionBlockSegment {
    Selector(SelectorTerm),
    Comma(CommaTerm),
}

impl ASTTerm for SelectionBlockSegment {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), VisitorError> {
        match self {
            Self::Selector(selector) => selector.accept(visitor)?,
            Self::Comma(comma) => comma.accept(visitor)?,
        };
        Ok(())
    }
}
