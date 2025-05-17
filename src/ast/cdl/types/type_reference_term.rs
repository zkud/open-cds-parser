use super::super::super::Location;
use super::super::IdentifierTerm;
use super::ArrayPrefix;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct TypeReferenceTerm {
    #[prop]
    location: Location,
    #[subnode_prop]
    array_prefix: Option<ArrayPrefix>,
    #[subnode_prop]
    type_details: Box<TypeDetailsVariant>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeDetailsVariant {
    Simple(IdentifierTerm),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::ASTTerm;

    #[derive(Default)]
    struct MockVisitor {
        pub identifiers: Vec<String>,
        pub keywords: Vec<Keyword>,
        pub punctuation: Vec<PunctuationSign>,
    }

    impl MockVisitor {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Visitor for MockVisitor {
        type Error = ();

        fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), Self::Error> {
            if let Some(term) = term.try_convert::<KeywordTerm>() {
                self.keywords.push(term.keyword().clone());
            } else if let Some(term) = term.try_convert::<PunctuationSignTerm>() {
                self.punctuation.push(term.sign().clone());
            } else if let Some(term) = term.try_convert::<IdentifierTerm>() {
                self.identifiers.push(term.full_name());
            }
            Ok(())
        }
    }

    #[test]
    fn with_simple_type_it_visits_identifier() {
        let location = Location::new_mock();
        let identifier = IdentifierTerm::new_basic(location.clone(), "TestType");
        let type_details = TypeDetailsVariant::Simple(identifier);
        let type_ref = TypeReferenceTerm::new(location, None, Box::new(type_details));
        let mut visitor = MockVisitor::new();

        let result = type_ref.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.identifiers, vec!["TestType"]);
        assert!(visitor.keywords.is_empty());
        assert!(visitor.punctuation.is_empty());
    }

    #[test]
    fn with_array_type_it_visits_array_prefix_and_identifier() {
        let location = Location::new_mock();
        let identifier = IdentifierTerm::new_basic(location.clone(), "TestType");
        let array_term = KeywordTerm::new(location.clone(), Keyword::Array);
        let of_term = KeywordTerm::new(location.clone(), Keyword::Of);
        let array_prefix = ArrayPrefix::ArrayOf {
            array: Box::new(array_term),
            of: Box::new(of_term),
        };
        let type_details = TypeDetailsVariant::Simple(identifier);
        let type_ref = TypeReferenceTerm::new(location, Some(array_prefix), Box::new(type_details));
        let mut visitor = MockVisitor::new();

        let result = type_ref.accept(&mut visitor);

        assert!(result.is_ok());
        assert_eq!(visitor.identifiers, vec!["TestType"]);
        assert_eq!(visitor.keywords, vec![Keyword::Array, Keyword::Of]);
        assert!(visitor.punctuation.is_empty());
    }

    #[test]
    fn with_structured_type_it_visits_braces() {
        let location = Location::new_mock();
        let open_brace =
            PunctuationSignTerm::new(location.clone(), PunctuationSign::OpenCurlyBrace);
        let close_brace =
            PunctuationSignTerm::new(location.clone(), PunctuationSign::CloseCurlyBrace);
        let structure = StructureTerm::new(
            location.clone(),
            Box::new(open_brace),
            vec![],
            Box::new(close_brace),
        );
        let type_details = TypeDetailsVariant::Structured(structure);
        let type_ref = TypeReferenceTerm::new(location, None, Box::new(type_details));
        let mut visitor = MockVisitor::new();

        let result = type_ref.accept(&mut visitor);

        assert!(result.is_ok());
        assert!(visitor.identifiers.is_empty());
        assert!(visitor.keywords.is_empty());
        assert_eq!(
            visitor.punctuation,
            vec![
                PunctuationSign::OpenCurlyBrace,
                PunctuationSign::CloseCurlyBrace
            ]
        );
    }
}
