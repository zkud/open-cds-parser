use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Default)]
#[ast_term(visitor_path = "process_name")]
pub struct NameTerm {
  #[prop]
  value: String,
}

#[cfg(test)]
mod tests {
  use super::super::super::common::mockable::tests::Mockable;
  use super::NameTerm;

  impl Mockable for NameTerm {
    fn variant_a() -> Self {
      NameTerm::new(String::variant_a())
    }

    fn variant_b() -> Self {
      NameTerm::new(String::variant_b())
    }
  }

  #[test]
  fn new_creates_the_term() {
    let term = NameTerm::new(
      String::variant_a()
    );
    assert_eq!(term.value(), &String::variant_a());
  }

  #[test]
  fn can_change_the_value() {
    let mut term = NameTerm::default();

    term.set_value(String::variant_b());

    assert_eq!(term.value(), &String::variant_b());
  }

  #[test]
  fn can_change_the_value_via_ref() {
    let mut term = NameTerm::default();

    *term.value_mut() = String::variant_b();

    assert_eq!(term.value(), &String::variant_b());
  }
}
