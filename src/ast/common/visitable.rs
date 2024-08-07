use std::{ops::Deref, sync::Arc};

use super::super::super::visitor::Visitor;

pub trait Visitable {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error>;
}

impl<A: Visitable> Visitable for Option<A> {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        if let Some(variant) = self {
            variant.accept(visitor)?;
        }
        Ok(())
    }
}

impl<A: Visitable> Visitable for Box<A> {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        self.deref().accept(visitor)?;
        Ok(())
    }
}

impl<A: Visitable> Visitable for Arc<A> {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        self.deref().accept(visitor)?;
        Ok(())
    }
}

impl<A: Visitable> Visitable for [A] {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        for term in self.iter() {
            term.accept(visitor)?;
        }
        Ok(())
    }
}
