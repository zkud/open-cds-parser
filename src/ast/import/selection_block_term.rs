use crate::ast::Visitable;

use super::super::{CommaTerm, PunctuationSignTerm, SelectorTerm};
use crate::ast::*;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct SelectionBlockTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    open_brace: Option<Box<PunctuationSignTerm>>,
    #[subnode_prop]
    selectors: Vec<SelectionBlockSegment>,
    #[subnode_prop]
    close_brace: Option<Box<PunctuationSignTerm>>,
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
