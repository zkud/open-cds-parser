use super::NameTerm;
use super::PathTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_using")]
pub struct UsingTerm {
    #[subnode_prop]
    selector: Box<NameTerm>,
    #[subnode_prop]
    path: Box<PathTerm>,
}
