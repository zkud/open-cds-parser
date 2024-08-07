use ast_term_derive::ASTTerm;

use super::{as_term::AsTerm, NameTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct AliasTerm {
    #[subnode_prop]
    as_term: Box<AsTerm>,
    #[subnode_prop]
    new_name: Box<NameTerm>,
}
