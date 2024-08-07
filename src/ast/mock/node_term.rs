#[cfg(test)]
use super::leaf_term::LeafTerm;
#[cfg(test)]
use ast_term_derive::ASTTerm;

#[cfg(test)]
#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct NodeTerm {
    #[subnode_prop]
    subnode: Box<LeafTerm>,
    #[prop]
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::ast_term::ASTTerm;

    // Helper function to create a mock LeafTerm
    fn create_mock_leaf_term() -> LeafTerm {
        LeafTerm::new(String::from("mock_leaf"))
    }

    #[test]
    fn test_node_term_new() {
        let leaf = Box::new(create_mock_leaf_term());
        let node = NodeTerm::new(leaf, String::from("test_value"));

        assert_eq!(node.value(), "test_value");
        assert_eq!(node.subnode().value(), "mock_leaf");
    }

    #[test]
    fn test_node_term_getters() {
        let leaf = Box::new(create_mock_leaf_term());
        let node = NodeTerm::new(leaf, String::from("test_value"));

        assert_eq!(node.value(), "test_value");
        assert_eq!(node.subnode().value(), "mock_leaf");
    }

    #[test]
    fn test_node_term_setters() {
        let leaf = Box::new(create_mock_leaf_term());
        let mut node = NodeTerm::new(leaf, String::from("test_value"));

        node.set_value(String::from("new_value"));
        assert_eq!(node.value(), "new_value");

        let new_leaf = Box::new(LeafTerm::new(String::from("new_leaf")));
        node.set_subnode(new_leaf);
        assert_eq!(node.subnode().value(), "new_leaf");
    }

    #[test]
    fn test_node_term_mutable_references() {
        let leaf = Box::new(create_mock_leaf_term());
        let mut node = NodeTerm::new(leaf, String::from("test_value"));

        *node.value_mut() = String::from("modified_value");
        assert_eq!(node.value(), "modified_value");

        node.subnode_mut().set_value(String::from("modified_leaf"));
        assert_eq!(node.subnode().value(), "modified_leaf");
    }

    #[test]
    fn test_node_term_accept() {
        struct MockVisitor {
            visited_node: bool,
            visited_leaf: bool,
        }

        impl crate::visitor::Visitor for MockVisitor {
            type Error = ();
            // Don't suppose any error handling here
            fn process<T: ASTTerm>(&mut self, term: &T) -> Result<(), ()> {
                if let Some(_) = term.try_convert::<NodeTerm>() {
                    self.visited_node = true;
                }
                if let Some(_) = term.try_convert::<LeafTerm>() {
                    self.visited_leaf = true;
                }
                Ok(())
            }
        }

        let leaf = Box::new(create_mock_leaf_term());
        let node = NodeTerm::new(leaf, String::from("test_value"));
        let mut visitor = MockVisitor {
            visited_node: false,
            visited_leaf: false,
        };

        assert!(node.accept(&mut visitor).is_ok());
        assert!(visitor.visited_node);
        assert!(visitor.visited_leaf);
    }
}
