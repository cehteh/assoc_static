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
/// This must be a macro because statics can not take template parameters from the outer scope.
///
///  * 'T' is the type you want have an static object associated to
///  * 'TARGET' is the type of the static object
///  * 'INIT' is used to initialize the static object
///
/// ```
/// use crate::assoc_static::*;
///
/// // define a type and attach a object to it
/// struct Example;
/// assoc_static!(Example, &'static str, "&str associated to Example");
///
/// // get it by type
/// assert_eq!(*Example::get_static(), "&str associated to Example");
///
/// // get it from an object
/// let example = Example;
/// assert_eq!(*AssocStatic::my_static(&example), "&str associated to Example");
/// ```
#[macro_export]
macro_rules! assoc_static {
    ($T:ty, $TARGET:ty, $INIT:expr) => {
        impl AssocStatic<$TARGET> for $T {
            fn get_static() -> &'static $TARGET {
                use std::marker::PhantomData;
                static STATIC_ASSOCIATED: ($TARGET, PhantomData<$TARGET>, PhantomData<$T>) =
                    ($INIT, PhantomData, PhantomData);
                &STATIC_ASSOCIATED.0
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::AssocStatic;

    struct TestType1;
    assoc_static!(TestType1, &'static str, "This is the first test type");

    #[test]
    fn smoke() {
        assert_eq!(*TestType1::get_static(), "This is the first test type");
    }

    struct TestType2;
    assoc_static!(TestType2, &'static str, "This is the second test type");
    assoc_static!(TestType2, u32, 42);

    #[test]
    fn multiple_statics() {
        assert_eq!(
            *<TestType2 as AssocStatic<&str>>::get_static(),
            "This is the second test type"
        );
        assert_eq!(*<TestType2 as AssocStatic<u32>>::get_static(), 42);
    }

    #[test]
    fn from_instance() {
        let test = TestType1;
        assert_eq!(
            *AssocStatic::my_static(&test),
            "This is the first test type"
        );
    }

    #[test]
    fn from_instance_multiple() {
        let test = TestType2;
        assert_eq!(
            *AssocStatic::<&str>::my_static(&test),
            "This is the second test type"
        );
        assert_eq!(*AssocStatic::<u32>::my_static(&test), 42);
    }
}
