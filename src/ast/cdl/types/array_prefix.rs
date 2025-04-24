use super::ArrayTerm;
use super::ManyTerm;
use super::OfTerm;
use crate::ast::{Visitable, Visitor};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArrayPrefix {
    ArrayOf {
        array: Box<ArrayTerm>,
        of: Box<OfTerm>,
    },
    Many(Box<ManyTerm>),
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
