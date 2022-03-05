#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

/// Associate a static object of type T to a type implementing this trait.
pub trait AssocStatic<T> {
    /// Returns a reference to the associated static object of the Self type
    fn get_static() -> &'static T;

    /// Returns a reference to the associated object from an instance.
    fn my_static(_this: &Self) -> &'static T {
        Self::get_static()
    }
}

/// Helper macro doing the boilerplate implementation.
#[macro_export]
macro_rules! assoc_static {
    ($TARGET:ty, $T:ty, $INIT:expr) => {
        impl AssocStatic<$TARGET> for $T {
            fn get_static() -> &'static $TARGET {
                use std::marker::PhantomData;
                static STATIC_ASSOCIATED: ($TARGET, PhantomData<$TARGET>, PhantomData<$T>) = ($INIT, PhantomData, PhantomData);
                &STATIC_ASSOCIATED.0
            }
        }
    };
}


#[cfg(test)]
mod tests {
    use crate::AssocStatic;

    struct TestType1;
    assoc_static!(&'static str, TestType1, "This is the first test type");

    #[test]
    fn smoke() {
        assert_eq!(*TestType1::get_static(), "This is the first test type");
    }

    struct TestType2;
    assoc_static!(&'static str, TestType2, "This is the second test type");
    assoc_static!(u32, TestType2, 42);

    #[test]
    fn multiple_statics() {
        assert_eq!(*<TestType2 as AssocStatic<&str>>::get_static(), "This is the second test type");
        assert_eq!(*<TestType2 as AssocStatic<u32>>::get_static(), 42);
    }

    #[test]
    fn from_instance() {
        let test = TestType1;
        assert_eq!(*AssocStatic::my_static(&test), "This is the first test type");
    }

    #[test]
    fn from_instance_multiple() {
        let test = TestType2;
        assert_eq!(*AssocStatic::<&str>::my_static(&test), "This is the second test type");
        assert_eq!(*AssocStatic::<u32>::my_static(&test), 42);
    }
}
