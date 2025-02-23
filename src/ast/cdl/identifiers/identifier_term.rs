use ast_term_derive::ASTTerm;

use super::super::super::DotTerm;
use super::SubIdentifierTerm;
use crate::ast::Location;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct IdentifierTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    segments: Vec<IdentifierSegment>,
}

impl IdentifierTerm {
    #[inline]
    pub fn new_basic(location: Location, name: &str) -> IdentifierTerm {
        IdentifierTerm::new(
            location.clone(),
            vec![IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                location.clone(),
                name.to_string(),
            ))],
        )
    }

    #[inline]
    pub fn full_name(&self) -> String {
        self.segments
            .iter()
            .map(|s| match s {
                IdentifierSegment::SubIdentifier(sub_ident) => sub_ident.value().clone(),
                IdentifierSegment::Dot(_) => ".".to_string(),
            })
            .reduce(|a, b| a + &b)
            .unwrap_or("".to_string())
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum IdentifierSegment {
    SubIdentifier(SubIdentifierTerm),
    Dot(DotTerm),
}

impl Visitable for IdentifierSegment {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::SubIdentifier(sub_identifier) => sub_identifier.accept(visitor)?,
            Self::Dot(dot) => dot.accept(visitor)?,
        };
        Ok(())
    }
}
