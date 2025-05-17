use super::super::super::KeywordTerm;
use crate::ast::{Visitable, Visitor};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ArrayPrefix {
    ArrayOf {
        array: Box<KeywordTerm>,
        of: Box<KeywordTerm>,
    },
    Many(Box<KeywordTerm>),
}

impl Visitable for ArrayPrefix {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            Self::ArrayOf { array, of } => {
                array.accept(visitor)?;
                of.accept(visitor)?;
            }
            Self::Many(term) => term.accept(visitor)?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::basic::Keyword;
    use crate::ast::common::{ASTTerm, Visitor};
    use crate::ast::Location;

    struct MockVisitor {
        pub visits: Vec<Keyword>,
    }

    impl MockVisitor {
        pub fn new() -> Self {
            MockVisitor { visits: vec![] }
        }
    }

    impl Visitor for MockVisitor {
        type Error = ();

        fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), Self::Error> {
            if let Some(term) = term.try_convert::<KeywordTerm>() {
                self.visits.push(term.keyword().clone());
            }
            Ok(())
        }
    }

    #[test]
    fn with_array_of_it_visits_both_keywords() {
        let location = Location::new_mock();
        let array_term = KeywordTerm::new(location.clone(), Keyword::Array);
        let of_term = KeywordTerm::new(location.clone(), Keyword::Of);
        let prefix = ArrayPrefix::ArrayOf {
            array: Box::new(array_term),
            of: Box::new(of_term),
        };
        let mut visitor = MockVisitor::new();

        let result = prefix.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec![Keyword::Array, Keyword::Of]);
    }

    #[test]
    fn with_many_it_visits_keyword() {
        let location = Location::new_mock();
        let many_term = KeywordTerm::new(location.clone(), Keyword::Many);
        let prefix = ArrayPrefix::Many(Box::new(many_term));
        let mut visitor = MockVisitor::new();

        let result = prefix.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.visits, vec![Keyword::Many]);
    }
}
