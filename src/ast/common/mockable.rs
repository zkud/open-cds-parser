#[cfg(test)]
pub mod tests {
  pub trait Mockable {
    fn variant_a() -> Self;
    fn variant_b() -> Self;
  }

  impl<M: Mockable> Mockable for Option<M> {
    fn variant_a() -> Self {
      Some(M::variant_a())
    }

    fn variant_b() -> Self {
      Some(M::variant_b())
    }
  }

  impl Mockable for String {
    fn variant_a() -> Self {
      String::from("a")
    }

    fn variant_b() -> Self {
      String::from("b")
    }
  }

  impl Mockable for bool {
    fn variant_a() -> Self {
      true
    }

    fn variant_b() -> Self {
      false
    }
  }
}
