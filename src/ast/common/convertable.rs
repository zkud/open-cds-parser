use std::any::Any;
use std::ops::Deref;
use std::sync::Arc;

pub trait Convertable: Clone + Any {
    fn try_convert<T: Convertable>(&self) -> Option<T>;
}

impl<A: Convertable> Convertable for Option<A> {
    fn try_convert<T: Convertable>(&self) -> Option<T> {
        self.as_ref().and_then(|o| o.try_convert())
    }
}

impl<A: Convertable> Convertable for Box<A> {
    fn try_convert<T: Convertable>(&self) -> Option<T> {
        self.deref().try_convert()
    }
}

impl<A: Convertable> Convertable for Arc<A> {
    fn try_convert<T: Convertable>(&self) -> Option<T> {
        self.deref().try_convert()
    }
}
