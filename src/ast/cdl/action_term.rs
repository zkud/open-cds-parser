use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(process_path = "process_action")]
pub struct ActionTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
  #[subnode_prop]
  params: Vec<ParamTerm>,
  #[subnode_prop]
  returned_type: Option<Box<ReturnsTerm>>,
}
