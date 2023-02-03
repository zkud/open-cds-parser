#[cfg(test)]
pub mod tests {
  pub trait Mockable {
    fn variant_a() -> Self;
    fn variant_b() -> Self;
  }

  impl Mockable for String {
    fn variant_a() -> Self {
      String::from("a")
    }

    fn variant_b() -> Self {
      String::from("b")
    }
  }
}