use super::*;

pub trait Visitor {
    type Error;

    fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), Self::Error>;
}
