use super::name_term::NameTerm;
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Default)]
#[ast_term(visitor_path = "process_param")]
pub struct ParamTerm {
  #[subnode_prop]
  name: Box<NameTerm>,
  #[subnode_prop]
  type_name: Box<NameTerm>,
}

#[cfg(test)]
mod tests {
  use super::super::super::common::mockable::tests::Mockable;
  use super::super::NameTerm;
  use super::ParamTerm;

  impl Mockable for ParamTerm {
    fn variant_a() -> Self {
      let name = NameTerm::new(String::variant_a());
      let type_name = NameTerm::new(String::variant_a());
      ParamTerm::new(Box::new(name), Box::new(type_name))
    }

    fn variant_b() -> Self {
      let name = NameTerm::new(String::variant_b());
      let type_name = NameTerm::new(String::variant_b());
      ParamTerm::new(Box::new(name), Box::new(type_name))
    }
  }

  #[test]
  fn new_creates_the_term() {
    let name = NameTerm::new(String::variant_a());
    let type_name = NameTerm::new(String::variant_a());
    let term = ParamTerm::new(Box::new(name), Box::new(type_name));
    assert_eq!(term.name(), &Box::new(NameTerm::variant_a()));
    assert_eq!(term.type_name(), &Box::new(NameTerm::variant_a()));
  }

  #[test]
  fn can_change_the_name() {
    let mut term = ParamTerm::default();
    let name = NameTerm::new(String::variant_a());

    term.set_name(Box::new(name));

    assert_eq!(term.name(), &Box::new(NameTerm::variant_a()));
  }

  #[test]
  fn can_change_the_name_via_ref() {
    let mut term = ParamTerm::default();
    let name = NameTerm::new(String::variant_a());

    *term.name_mut() = Box::new(name);

    assert_eq!(term.name(), &Box::new(NameTerm::variant_a()));
  }

  #[test]
  fn can_change_the_type_name() {
    let mut term = ParamTerm::default();
    let type_name = NameTerm::new(String::variant_a());

    term.set_type_name(Box::new(type_name));

    assert_eq!(term.type_name(), &Box::new(NameTerm::variant_a()));
  }

  #[test]
  fn can_change_the_type_name_via_ref() {
    let mut term = ParamTerm::default();
    let type_name = NameTerm::new(String::variant_a());

    *term.type_name_mut() = Box::new(type_name);

    assert_eq!(term.type_name(), &Box::new(NameTerm::variant_a()));
  }
}
