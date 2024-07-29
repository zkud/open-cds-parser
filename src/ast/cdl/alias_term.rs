use ast_term_derive::ASTTerm;

use super::{as_term::AsTerm, NameTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_alias")]
pub struct AliasTerm {
    #[subnode_prop]
    as_term: Box<AsTerm>,
    #[subnode_prop]
    new_name: Box<NameTerm>,
}
