use super::super::super::Location;
use super::SimpleTypeDetailsTerm;
use super::StructuredTypeDetailsTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct TypeReferenceTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    type_details: Box<TypeDetailsVariant>,
    #[prop]
    is_arrayed: bool,
}

impl TypeReferenceTerm {
    pub fn new_scalar(
        location: Location,
        type_details: Box<TypeDetailsVariant>,
    ) -> TypeReferenceTerm {
        TypeReferenceTerm {
            location,
            type_details,
            is_arrayed: false,
        }
    }

    pub fn new_arrayed(
        location: Location,
        type_details: Box<TypeDetailsVariant>,
    ) -> TypeReferenceTerm {
        TypeReferenceTerm {
            location,
            type_details,
            is_arrayed: true,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeDetailsVariant {
    Simple(SimpleTypeDetailsTerm),
    Structured(StructuredTypeDetailsTerm),
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
