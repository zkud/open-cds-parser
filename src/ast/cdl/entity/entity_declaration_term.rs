use crate::ast::IdentifierTerm;
use crate::ast::KeywordTerm;
use crate::ast::Location;
use crate::ast::PunctuationSignTerm;
use crate::ast::StructureTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct EntityDeclarationTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    define: Option<Box<KeywordTerm>>,
    #[subnode_prop]
    entity: Box<KeywordTerm>,
    #[subnode_prop]
    identifier: Box<IdentifierTerm>,
    #[subnode_prop]
    colon: Option<Box<PunctuationSignTerm>>,
    #[subnode_prop]
    applied_aspects: Vec<AppliedAspectSegment>,
    #[subnode_prop]
    structure: Box<StructureTerm>,
    #[subnode_prop]
    semicolumn: Option<Box<PunctuationSignTerm>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AppliedAspectSegment {
    Aspect(IdentifierTerm),
    Comma(PunctuationSignTerm),
}

impl Visitable for AppliedAspectSegment {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::Aspect(aspect) => aspect.accept(visitor)?,
            Self::Comma(comma) => comma.accept(visitor)?,
        };
        Ok(())
    }
}

impl AppliedAspectSegment {
    // Helper method for tests
    #[cfg(test)]
    fn new_aspect(name: &str) -> Self {
        Self::Aspect(IdentifierTerm::new_basic(Location::new_mock(), name))
    }

    // Helper method for tests
    #[cfg(test)]
    fn new_comma() -> Self {
        Self::Comma(PunctuationSignTerm::new(
            Location::new_mock(),
            PunctuationSign::Comma,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::{ASTTerm, Visitor};
    use crate::ast::Location;
    use crate::ast::PunctuationSign;

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
            if let Some(term) = term.try_convert::<IdentifierTerm>() {
                self.visits.push(term.full_name());
            } else if let Some(_) = term.try_convert::<PunctuationSignTerm>() {
                self.visits.push(",".to_string());
            }
            Ok(())
        }
    }

    #[test]
    fn with_aspect_it_visits() {
        let segment = AppliedAspectSegment::new_aspect("TestAspect");
        let mut visitor = MockVisitor::new();

        let result = segment.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["TestAspect"]);
    }

    #[test]
    fn with_comma_it_visits() {
        let segment = AppliedAspectSegment::new_comma();
        let mut visitor = MockVisitor::new();

        let result = segment.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec![","]);
    }

    #[test]
    fn with_multiple_segments_it_visits_all() {
        let segments = vec![
            AppliedAspectSegment::new_aspect("FirstAspect"),
            AppliedAspectSegment::new_comma(),
            AppliedAspectSegment::new_aspect("SecondAspect"),
        ];
        let mut visitor = MockVisitor::new();

        for segment in segments {
            let result = segment.accept(&mut visitor);
            assert!(result.is_ok());
        }

        assert_eq!(visitor.visits, vec!["FirstAspect", ",", "SecondAspect"]);
    }
}
