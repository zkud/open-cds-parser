use ast_term_derive::ASTTerm;

use super::{NameTerm, SelectAllTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct ImportIdentifierTerm {
    #[subnode_prop]
    base_name: Box<NameTerm>,
    #[subnode_prop]
    all_selector: Option<Box<SelectAllTerm>>,
}
