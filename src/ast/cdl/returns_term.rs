use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug)]
#[ast_term(visitor_path = "process_returns", generate_new = false)]
pub struct ReturnsTerm {
  #[subnode_prop]
  type_name: Box<NameTerm>,
  #[prop]
  is_arrayed: bool,
}

impl ReturnsTerm {
  pub fn new(type_name: Box<NameTerm>) -> ReturnsTerm {
    ReturnsTerm {
      type_name,
      is_arrayed: false,
    }
  }

  pub fn new_arrayed(type_name: Box<NameTerm>) -> ReturnsTerm {
    ReturnsTerm {
      type_name,
      is_arrayed: true,
    }
  }
}
