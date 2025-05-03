use super::super::super::KeywordTerm;
use super::OfTerm;
use crate::ast::{Visitable, Visitor};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArrayPrefix {
    ArrayOf {
        array: Box<KeywordTerm>,
        of: Box<OfTerm>,
    },
    Many(Box<KeywordTerm>),
}

impl Visitable for ArrayPrefix {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::ArrayOf { array, of } => {
                array.accept(visitor)?;
                of.accept(visitor)?;
            }
            Self::Many(term) => term.accept(visitor)?,
        };
        Ok(())
    }
}
