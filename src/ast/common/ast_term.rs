use super::{Convertable, Visitable};

pub trait ASTTerm: Visitable + Convertable {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::LeafTerm, visitor::Visitor};

    use std::sync::Arc;

    struct MockVisitor {
        pub visits: Vec<String>,
    }

    impl MockVisitor {
        pub fn new() -> Self {
            MockVisitor { visits: vec![] }
        }
    }

    #[derive(Debug, PartialEq)]
    struct VisitorError;

    impl Visitor for MockVisitor {
        type Error = ();

        // Don't suppose any errors to exist here
        fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), ()> {
            if let Some(term) = term.try_convert::<LeafTerm>() {
                self.visits.push(term.value().clone());
            }
            Ok(())
        }
    }

    #[test]
    fn test_deref_ast_term() {
        let term = Box::new(LeafTerm::new("1".to_string()));

        let mut visitor = MockVisitor::new();
        let result = term.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["1"]);
    }

    #[test]
    fn test_option_ast_term_some() {
        let term = Some(LeafTerm::new("2".to_string()));

        let mut visitor = MockVisitor::new();
        let result = term.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["2"]);
    }

    #[test]
    fn test_option_ast_term_none() {
        let term: Option<LeafTerm> = None;

        let mut visitor = MockVisitor::new();
        let result = term.accept(&mut visitor);

        assert!(result.is_ok());
        assert!(visitor.visits.is_empty());
    }

    #[test]
    fn test_box_ast_term() {
        let term = Box::new(LeafTerm::new("3".to_string()));

        let mut visitor = MockVisitor::new();
        let result = term.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["3"]);
    }

    #[test]
    fn test_arc_ast_term() {
        let term = Arc::new(LeafTerm::new("4".to_string()));

        let mut visitor = MockVisitor::new();
        let result = term.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["4"]);
    }
}
