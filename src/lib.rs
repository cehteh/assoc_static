#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

/// Associates a static object of type T and a marker TAG.
/// Use the `assoc_static!()` macro for implemeting this trait on types.
pub trait AssocStatic<T, TAG> {
    /// Returns a reference to the associated static object of the Self type
    fn get_static() -> &'static T;

    /// Returns a reference to the associated object from an instance.
    fn from(_this: &Self) -> &'static T {
        Self::get_static()
    }
}

/// Helper macro doing the boilerplate implementation.
/// This must be a macro because statics can not take template parameters from the outer scope.
///
///  * 'T' is the type you want have an static object associated to
///  * 'TAG' A type marker to discriminate this implementation, defaults to ()
///  * 'TARGET' is the type of the static object
///  * 'INIT' is used to initialize the static object
///
/// The simple case, associate something to some local type:
/// ```
/// use crate::assoc_static::*;
///
/// // define a type and attach a '&str' object to it
/// struct Example;
/// assoc_static!(Example, &'static str, "&str associated to Example");
///
/// // get it by type
/// assert_eq!(*Example::get_static(), "&str associated to Example");
///
/// // get it from an object
/// let example = Example;
/// assert_eq!(*AssocStatic::from(&example), "&str associated to Example");
/// ```
///
/// The 'TAG' is required when one needs to disambiguate between different target values of
/// the same type or when an association between foreign types not defined in the current
/// crate shall be established. This can be any (non-generic) type your crate defines,
/// preferably you just make a zero-size struct just for this purpose. It is only used as
/// marker for disambiguation.
///
/// Disambiguate between different static objects:
/// ```
/// use crate::assoc_static::*;
///
/// struct Example;
///
/// // attach a '&str' object to Example
/// struct Hello;
/// assoc_static!(Example, Hello, &'static str, "Hello World!");
///
/// // again bit for another purpose
/// struct ExplainType;
/// assoc_static!(Example, ExplainType, &'static str, "This is 'struct Example'");
///
/// let example = Example;
///
/// // resolve the disambiguity with a turbofish
/// assert_eq!(*AssocStatic::<&str, Hello>::from(&example), "Hello World!");
/// assert_eq!(*AssocStatic::<&str, ExplainType>::from(&example), "This is 'struct Example'");
/// ```
///
/// Make an association between foreign types:
/// ```
/// use crate::assoc_static::*;
///
/// // attach a '&str' object to i32
/// struct I32ExampleStr;
/// assoc_static!(i32, I32ExampleStr, &'static str, "&str associated to i32");
///
/// // get it
/// assert_eq!(*AssocStatic::from(&100i32), "&str associated to i32");
/// ```
#[macro_export]
macro_rules! assoc_static {
    ($T:ty, $TAG:ty, $TARGET:ty, $INIT:expr) => {
        impl $crate::AssocStatic<$TARGET, $TAG> for $T {
            fn get_static() -> &'static $TARGET {
                static ASSOCIATED_STATIC: (
                    $TARGET,
                    std::marker::PhantomData<$crate::MakeSync<$T>>,
                    std::marker::PhantomData<$crate::MakeSync<$TAG>>,
                ) = ($INIT, std::marker::PhantomData, std::marker::PhantomData);
                &ASSOCIATED_STATIC.0
            }
        }
    };
    ($T:ty, $TARGET:ty, $INIT:expr) => {
        impl $crate::AssocStatic<$TARGET, ()> for $T {
            fn get_static() -> &'static $TARGET {
                static ASSOCIATED_STATIC: (
                    $TARGET,
                    std::marker::PhantomData<$crate::MakeSync<$T>>,
                    std::marker::PhantomData<()>,
                ) = ($INIT, std::marker::PhantomData, std::marker::PhantomData);
                &ASSOCIATED_STATIC.0
            }
        }
    };
}

/// Only a helper, needs to be public because of the macro
#[doc(hidden)]
pub struct MakeSync<T>(T);
unsafe impl<T> Sync for MakeSync<T> {}

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
            *<TestType2 as AssocStatic<&str, ()>>::get_static(),
            "This is the second test type"
        );
        assert_eq!(*<TestType2 as AssocStatic<u32, ()>>::get_static(), 42);
    }

    #[test]
    fn from_instance() {
        let test = TestType1;
        assert_eq!(*AssocStatic::from(&test), "This is the first test type");
    }

    #[test]
    fn from_instance_multiple() {
        let test = TestType2;
        assert_eq!(
            *AssocStatic::<&str, _>::from(&test),
            "This is the second test type"
        );
        assert_eq!(*AssocStatic::<u32, _>::from(&test), 42);
    }
}
