use super::super::super::KeywordTerm;
use crate::ast::{Visitable, Visitor};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArrayPrefix {
    ArrayOf {
        array: Box<KeywordTerm>,
        of: Box<KeywordTerm>,
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
