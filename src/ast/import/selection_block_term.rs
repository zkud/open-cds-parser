use crate::ast::Visitable;

use super::super::super::visitor::Visitor;
use super::super::common::ast_term::ASTTerm;
use super::super::{CloseCurlyBraceTerm, CommaTerm, OpenCurlyBraceTerm, SelectorTerm};
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
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

impl Visitable for SelectionBlockSegment {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::Selector(selector) => selector.accept(visitor)?,
            Self::Comma(comma) => comma.accept(visitor)?,
        };
        Ok(())
    }
}
