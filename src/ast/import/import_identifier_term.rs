use ast_term_derive::ASTTerm;

use crate::ast::*;

use super::super::{IdentifierTerm, PunctuationSignTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ImportIdentifierTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    variant: Box<ImportIdentifierVariant>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ImportIdentifierVariant {
    IdentifierOnly(Box<IdentifierTerm>),
    IdentifierWithWildcart {
        identifier: Box<IdentifierTerm>,
        dot: Box<PunctuationSignTerm>,
        wildcart: Box<PunctuationSignTerm>,
    },
    SelectAll(Box<PunctuationSignTerm>),
}

impl Visitable for ImportIdentifierVariant {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::IdentifierOnly(identifier) => identifier.accept(visitor)?,
            Self::IdentifierWithWildcart {
                identifier,
                dot,
                wildcart,
            } => {
                identifier.accept(visitor)?;
                dot.accept(visitor)?;
                wildcart.accept(visitor)?;
            }
            Self::SelectAll(wildcart) => wildcart.accept(visitor)?,
        };
        Ok(())
    }
}
