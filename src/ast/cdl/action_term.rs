use super::name_term::NameTerm;
use super::param_term::ParamTerm;
use super::returns_term::ReturnsTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Default)]
#[ast_term(visitor_path = "process_action")]
pub struct ActionTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
  #[subnode_prop]
  params: Vec<ParamTerm>,
  #[subnode_prop]
  returned_type: Option<Box<ReturnsTerm>>,
}

#[cfg(test)]
mod tests {
  use super::super::name_term::NameTerm;
  use super::super::param_term::ParamTerm;
  use super::super::returns_term::ReturnsTerm;
  use super::ActionTerm;

  #[test]
  fn new_creates_the_term() {
    let param_name = NameTerm::new("my_param".to_string());
    let param_type_name = NameTerm::new("ParamTypeName".to_string());
    let param = ParamTerm::new(Box::new(param_name), Box::new(param_type_name));
    let action_term = ActionTerm::new(
      Box::new(NameTerm::new("my_function".to_string())),
      vec![param],
      Some(Box::new(ReturnsTerm::new(Box::new(NameTerm::new(
        "bool".to_string(),
      ))))),
    );
    assert_eq!(action_term.name().value(), "my_function");
    assert_eq!(action_term.params()[0].name().value(), "my_param");
    assert_eq!(action_term.params()[0].type_name().value(), "ParamTypeName");
    assert_eq!(
      action_term
        .returned_type()
        .as_ref()
        .unwrap()
        .type_name()
        .value(),
      "bool"
    );
  }

  #[test]
  fn can_change_the_name() {
    let mut action_term = ActionTerm::default();
    let name = NameTerm::new(String::from("New"));

    action_term.set_name(Box::new(name));

    assert_eq!(action_term.name().value(), "New");
  }

  #[test]
  fn can_change_the_name_via_ref() {
    let mut action_term = ActionTerm::default();
    let name = NameTerm::new(String::from("New"));

    *action_term.name_mut() = Box::new(name);

    assert_eq!(action_term.name().value(), "New");
  }

  #[test]
  fn can_change_the_param() {
    let mut action_term = ActionTerm::default();
    let param_name = NameTerm::new("my_param".to_string());
    let param_type_name = NameTerm::new("ParamTypeName".to_string());
    let param = ParamTerm::new(Box::new(param_name), Box::new(param_type_name));

    action_term.set_params(vec![param]);

    assert_eq!(action_term.params()[0].name().value(), "my_param");
  }

  #[test]
  fn can_change_the_param_via_ref() {
    let mut action_term = ActionTerm::default();
    let param_name = NameTerm::new("my_param".to_string());
    let param_type_name = NameTerm::new("ParamTypeName".to_string());
    let param = ParamTerm::new(Box::new(param_name), Box::new(param_type_name));

    *action_term.params_mut() = vec![param];

    assert_eq!(action_term.params()[0].name().value(), "my_param");
  }

  #[test]
  fn can_change_the_return_type() {
    let mut action_term = ActionTerm::default();
    let return_type = Some(Box::new(ReturnsTerm::new(Box::new(NameTerm::new(
      "bool".to_string(),
    )))));

    action_term.set_returned_type(return_type);

    assert_eq!(
      action_term
        .returned_type()
        .as_ref()
        .unwrap()
        .type_name()
        .value(),
      "bool"
    );
  }

  #[test]
  fn can_change_the_return_type_via_ref() {
    let mut action_term = ActionTerm::default();
    let return_type = Some(Box::new(ReturnsTerm::new(Box::new(NameTerm::new(
      "bool".to_string(),
    )))));

    action_term.set_returned_type(return_type);

    assert_eq!(
      action_term
        .returned_type()
        .as_ref()
        .unwrap()
        .type_name()
        .value(),
      "bool"
    );
  }
}
