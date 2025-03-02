use ast_term_derive::ASTTerm;

use crate::ast::*;

use super::super::{DotTerm, IdentifierTerm, WildcartTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ImportIdentifierTerm {
    location: Location,
    #[subnode_prop]
    variant: Box<ImportIdentifierVariant>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ImportIdentifierVariant {
    IdentifierOnly(Box<IdentifierTerm>),
    IdentifierWithWildcart {
        identifier: Box<IdentifierTerm>,
        dot: Box<DotTerm>,
        wildcart: Box<WildcartTerm>,
    },
    SelectAll(Box<WildcartTerm>),
}

impl Visitable for ImportIdentifierVariant {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::IdentifierOnly(name) => name.accept(visitor)?,
            Self::IdentifierWithWildcart {
                identifier: name,
                dot,
                wildcart,
            } => {
                name.accept(visitor)?;
                dot.accept(visitor)?;
                wildcart.accept(visitor)?;
            }
            Self::SelectAll(wildcart) => wildcart.accept(visitor)?,
        };
        Ok(())
    }
}
