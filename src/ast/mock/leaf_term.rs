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
}
