#[cfg(test)]
use ast_term_derive::ASTTerm;

#[cfg(test)]
#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct LeafTerm {
    #[prop]
    value: String,
}

#[cfg(test)]
mod tests {
    use super::LeafTerm;
    use ast_term_derive::ASTTerm;

    #[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
    struct AnotherTerm {}

    #[test]
    fn test_value() {
        let name_term = LeafTerm::new("TestName".to_string());
        assert_eq!(name_term.value(), "TestName");
    }

    #[test]
    fn test_value_mut() {
        let mut name_term = LeafTerm::new("TestName".to_string());
        *name_term.value_mut() = "UpdatedName".to_string();
        assert_eq!(name_term.value(), "UpdatedName");
    }

    #[test]
    fn test_set_value() {
        let mut name_term = LeafTerm::new("TestName".to_string());
        name_term.set_value("NewName".to_string());
        assert_eq!(name_term.value(), "NewName");
    }

    #[test]
    fn test_new() {
        let name_term = LeafTerm::new("TestName".to_string());
        assert_eq!(name_term.value(), "TestName");
    }

    #[test]
    fn with_the_same_type_convert_returns_term() {
        let term = LeafTerm::new("any value".to_string());

        let term = term.try_convert::<LeafTerm>();

        assert!(term.is_some());
    }

    #[test]
    fn with_another_type_convert_returns_none() {
        let term = LeafTerm::new("any value".to_string());

        let term = term.try_convert::<AnotherTerm>();

        assert!(term.is_none());
    }
}
