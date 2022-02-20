/*!
This library provides some utility traits to make working with [`Any`] smoother.
This crate contains similiar functionality to the `downcast` crate, but simpler,
e.g. it isn't necessary to call some macro to instantiate the downcast methods.

# Usage example
```
use as_any::{AsAny, Downcast};

struct Test;

trait Custom: AsAny {
    // whatever you like to put inside of your trait
}

impl Custom for Test {}

fn lol() {
    let x = Test;
    let y: &dyn Custom = &x;
    // With (extension) trait `Downcast` in scope.
    y.downcast_ref::<Test>().unwrap();
}
```
**/

#![no_std]
#![forbid(unsafe_code)]
use core::any::Any;

/// This trait is an extension trait to [`Any`], and adds methods to retrieve a `&dyn Any`
pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Gets the type name of `self`
    fn type_name(&self) -> &'static str;
}

impl<T: Any> AsAny for T {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline(always)]
    fn type_name(&self) -> &'static str {
        core::any::type_name::<T>()
    }
}

/// This is a shim around `AaAny` to avoid some boilerplate code.
/// It is a separate trait because it is also implemented
/// on runtime polymorphic traits (which are `!Sized`).
pub trait Downcast: AsAny {
    /// Returns `true` if the boxed type is the same as `T`.
    ///
    /// Forward to the method defined on the type `Any`.
    #[inline]
    fn is<T>(&self) -> bool
    where
        T: AsAny,
    {
        self.as_any().is::<T>()
    }

    /// Forward to the method defined on the type `Any`.
    #[inline]
    fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: AsAny,
    {
        self.as_any().downcast_ref()
    }

    /// Forward to the method defined on the type `Any`.
    #[inline]
    fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: AsAny,
    {
        self.as_any_mut().downcast_mut()
    }
}

impl<T: ?Sized + AsAny> Downcast for T {}
