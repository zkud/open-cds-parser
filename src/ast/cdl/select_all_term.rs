use ast_term_derive::ASTTerm;

use super::{dot_term::DotTerm, wildcart_term::WildcartTerm};

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
pub struct SelectAllTerm {
    #[subnode_prop]
    dot: Box<DotTerm>,
    #[subnode_prop]
    wildcart: Box<WildcartTerm>,
}
