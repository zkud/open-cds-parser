use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(PartialEq, Eq, Debug, ASTTerm)]
#[ast_term(process_path = "process_returns", generate_news = false)]
pub struct ReturnsTerm {
  #[subnode_prop]
  type_name: Box<NameTerm>,
  #[prop]
  is_arrayed: bool,
}

impl ReturnsTerm {
  pub fn new_boxed(type_name: Box<NameTerm>) -> Box<ReturnsTerm> {
    Box::new(ReturnsTerm::new(type_name))
  }

  pub fn new(type_name: Box<NameTerm>) -> ReturnsTerm {
    ReturnsTerm {
      type_name,
      is_arrayed: false,
    }
  }

  pub fn new_arrayed_boxed(type_name: Box<NameTerm>) -> Box<ReturnsTerm> {
    Box::new(ReturnsTerm::new_arrayed(type_name))
  }

  pub fn new_arrayed(type_name: Box<NameTerm>) -> ReturnsTerm {
    ReturnsTerm {
      type_name,
      is_arrayed: true,
    }
  }
}
