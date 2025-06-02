use super::super::PunctuationSignTerm;
use super::SubIdentifierTerm;
use crate::ast::Location;
use ast_term_derive::ASTTerm;

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
    Dot(PunctuationSignTerm),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{ASTTerm, Visitor};
    use crate::ast::Location;

    struct MockVisitor {
        pub visits: Vec<String>,
    }

    impl MockVisitor {
        pub fn new() -> Self {
            MockVisitor { visits: vec![] }
        }
    }

    impl Visitor for MockVisitor {
        type Error = ();

        fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), Self::Error> {
            if let Some(term) = term.try_convert::<SubIdentifierTerm>() {
                self.visits.push(term.value().clone());
            } else if let Some(_) = term.try_convert::<PunctuationSignTerm>() {
                self.visits.push(".".to_string());
            }
            Ok(())
        }
    }

    #[test]
    fn with_sub_identifier_it_visits() {
        let location = Location::new_mock();
        let term = IdentifierTerm::new_basic(location.clone(), "test");
        let mut visitor = MockVisitor::new();

        let result = term.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["test"]);
    }

    #[test]
    fn with_dot_it_visits() {
        let location = Location::new_mock();
        let dot_term = PunctuationSignTerm::new(location.clone(), PunctuationSign::Dot);
        let term = IdentifierTerm::new(location, vec![IdentifierSegment::Dot(dot_term)]);
        let mut visitor = MockVisitor::new();

        let result = term.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["."]);
    }

    #[test]
    fn with_all_variants_it_visits() {
        let location = Location::new_mock();
        let sub_identifier = SubIdentifierTerm::new(location.clone(), "test".to_string());
        let dot_term = PunctuationSignTerm::new(location.clone(), PunctuationSign::Dot);
        let term = IdentifierTerm::new(
            location,
            vec![
                IdentifierSegment::SubIdentifier(sub_identifier.clone()),
                IdentifierSegment::Dot(dot_term),
                IdentifierSegment::SubIdentifier(sub_identifier),
            ],
        );
        let mut visitor = MockVisitor::new();

        let result = term.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["test", ".", "test"]);
    }

    #[test]
    fn with_basic_identifier_it_returns_full_name() {
        let location = Location::new_mock();
        let term = IdentifierTerm::new_basic(location, "test");

        assert_eq!(term.full_name(), "test");
    }

    #[test]
    fn with_namespaced_identifier_it_returns_full_name() {
        let location = Location::new_mock();
        let term = IdentifierTerm::new(
            location,
            vec![
                IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                    Location::new_mock(),
                    "parent1".to_string(),
                )),
                IdentifierSegment::Dot(PunctuationSignTerm::new(
                    Location::new_mock(),
                    PunctuationSign::Dot,
                )),
                IdentifierSegment::SubIdentifier(SubIdentifierTerm::new(
                    Location::new_mock(),
                    "parent2".to_string(),
                )),
            ],
        );

        assert_eq!(term.full_name(), "parent1.parent2");
    }

    #[test]
    fn with_empty_segments_it_returns_empty_string() {
        let location = Location::new_mock();
        let term = IdentifierTerm::new(location, vec![]);

        assert_eq!(term.full_name(), "");
    }
}
