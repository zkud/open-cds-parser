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

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;
    use std::sync::Arc;

    // Helper structs for testing
    #[derive(Clone)]
    struct A(i32);

    #[derive(Clone)]
    struct B(i32);

    impl Convertable for A {
        fn try_convert<T: Convertable>(&self) -> Option<T> {
            let self_any = self as &dyn std::any::Any;
            self_any.downcast_ref::<T>().cloned()
        }
    }

    impl Convertable for B {
        fn try_convert<T: Convertable>(&self) -> Option<T> {
            let self_any = self as &dyn std::any::Any;
            self_any.downcast_ref::<T>().cloned()
        }
    }

    #[test]
    fn test_direct_conversion() {
        let a = A(42);
        let b: Option<B> = a.try_convert();
        assert!(b.is_none());
    }

    #[test]
    fn test_option_conversion() {
        let a = Some(A(42));
        let b: Option<B> = a.try_convert();
        assert!(b.is_none());

        let none: Option<A> = None;
        let b: Option<B> = none.try_convert();
        assert!(b.is_none());
    }

    #[test]
    fn test_box_conversion() {
        let a = Box::new(A(42));
        let b: Option<B> = a.try_convert();
        assert!(b.is_none());
    }

    #[test]
    fn test_arc_conversion() {
        let a = Arc::new(A(42));
        let b: Option<B> = a.try_convert();
        assert!(b.is_none());
    }

    #[test]
    fn test_successful_conversion() {
        let a = A(42);
        let a_as_any: &dyn Any = &a;
        let a_converted: Option<A> = a_as_any.downcast_ref::<A>().map(|a| a.clone());
        assert!(a_converted.is_some());
        assert_eq!(a_converted.unwrap().0, 42);
    }

    #[test]
    fn test_nested_option_conversion() {
        let a = Some(Some(A(42)));
        let b: Option<B> = a.try_convert();
        assert!(b.is_none());
    }

    #[test]
    fn test_nested_box_conversion() {
        let a = Box::new(Box::new(A(42)));
        let b: Option<B> = a.try_convert();
        assert!(b.is_none());
    }

    #[test]
    fn test_nested_arc_conversion() {
        let a = Arc::new(Arc::new(A(42)));
        let b: Option<B> = a.try_convert();
        assert!(b.is_none());
    }
}
