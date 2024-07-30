use super::super::super::visitor::Visitor;
use super::super::common::ast_term::ASTTerm;
use super::{CloseCurlyBraceTerm, CommaTerm, OpenCurlyBraceTerm, SelectorTerm};
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_selection_block")]
pub struct SelectionBlockTerm {
    #[subnode_prop]
    open_brace: Option<Box<OpenCurlyBraceTerm>>,
    #[subnode_prop]
    selectors: Vec<SelectionBlockSegment>,
    #[subnode_prop]
    close_brace: Option<Box<CloseCurlyBraceTerm>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SelectionBlockSegment {
    Selector(SelectorTerm),
    Comma(CommaTerm),
}

impl ASTTerm for SelectionBlockSegment {
    fn accept<E>(&self, visitor: &mut dyn Visitor<E>) -> Result<(), E> {
        match self {
            Self::Selector(selector) => selector.accept(visitor)?,
            Self::Comma(comma) => comma.accept(visitor)?,
        };
        Ok(())
    }
}
