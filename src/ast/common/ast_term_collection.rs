use super::Visitable;

pub trait ASTTermCollection: Visitable {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{ASTTerm, LeafTerm},
        visitor::Visitor,
    };

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
    fn test_slice_ast_term() {
        let terms = vec![
            LeafTerm::new("5".to_string()),
            LeafTerm::new("6".to_string()),
            LeafTerm::new("7".to_string()),
        ];

        let mut visitor = MockVisitor::new();
        let result = terms.as_slice().accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec!["5", "6", "7"]);
    }

    #[test]
    fn test_empty_slice_ast_term() {
        let terms: Vec<LeafTerm> = vec![];

        let mut visitor = MockVisitor::new();
        let result = terms.as_slice().accept(&mut visitor);

        assert!(result.is_ok());
        assert!(visitor.visits.is_empty());
    }
}
