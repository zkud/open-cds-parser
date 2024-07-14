use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug)]
#[ast_term(visitor_path = "process_param")]
pub struct ParamTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
  #[subnode_prop]
  type_name: Box<NameTerm>,
}
