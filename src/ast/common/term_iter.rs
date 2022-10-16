use super::reference::Reference;
use std::fmt::Debug;
use std::ops::Deref;

pub struct TermIter<'i, T> {
  iter: Box<dyn Iterator<Item = &'i T> + 'i>,
}

impl<'i, T: PartialEq + Eq + Debug> Iterator for TermIter<'i, T> {
  type Item = &'i T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next()
  }
}

impl<'i, T: PartialEq + Eq + Debug> TermIter<'i, T> {
  pub fn new_from_deref_vec<D: Deref<Target = T>>(iterable: &'i Vec<D>) -> Self {
    let iterator = iterable.iter();
    let iterator = iterator.map(|x| x.deref());
    TermIter {
      iter: Box::new(iterator),
    }
  }

  pub fn new_from_referenced_deref_vec<D: Deref<Target = T> + PartialEq + Eq + Debug>(
    iterable: &'i Vec<Reference<D>>,
  ) -> Self {
    let iterator = iterable.iter();
    let iterator = iterator.map(|x| x.value_ref().deref());
    TermIter {
      iter: Box::new(iterator),
    }
  }
}
