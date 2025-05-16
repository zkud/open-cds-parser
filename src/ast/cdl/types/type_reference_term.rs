use super::super::super::Location;
use super::ArrayPrefix;
use super::SimpleTypeDetailsTerm;
use super::StructuredTypeDetailsTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct TypeReferenceTerm {
    #[prop]
    location: Location,
    #[prop]
    array_prefix: Option<ArrayPrefix>,
    #[subnode_prop]
    type_details: Box<TypeDetailsVariant>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeDetailsVariant {
    Simple(SimpleTypeDetailsTerm),
    Structured(StructureTerm),
}

impl Visitable for TypeDetailsVariant {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::Simple(term) => term.accept(visitor)?,
            Self::Structured(term) => term.accept(visitor)?,
        };
        Ok(())
    }
}
