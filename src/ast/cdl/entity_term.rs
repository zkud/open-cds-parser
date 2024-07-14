use super::field_term::FieldTerm;
use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug)]
#[ast_term(visitor_path = "process_entity")]
pub struct EntityTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
  #[subnode_prop]
  applied_aspects: Vec<NameTerm>,
  #[subnode_prop]
  fields: Vec<FieldTerm>,
}
