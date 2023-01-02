use super::super::cdl::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(visitor_path = "process_using_ref")]
pub struct UsingRefTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
}
