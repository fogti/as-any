/*!
This library provides some utility traits to make working with [`Any`] smoother.
This crate contains similiar functionality to the `downcast` crate, but simpler.

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

use core::{any::Any, fmt};

/// This trait is an extension trait to [`Any`], and adds methods to retrieve a `&dyn Any`
pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    #[doc(hidden)]
    fn as_my_any(&self) -> &dyn AsAny;

    #[doc(hidden)]
    fn as_my_any_mut(&mut self) -> &mut dyn AsAny;

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

    #[doc(hidden)]
    #[inline(always)]
    fn as_my_any(&self) -> &dyn AsAny {
        self
    }

    #[doc(hidden)]
    #[inline(always)]
    fn as_my_any_mut(&mut self) -> &mut dyn AsAny {
        self
    }

    #[inline(always)]
    fn type_name(&self) -> &'static str {
        core::any::type_name::<T>()
    }
}

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

macro_rules! implement {
    ($base:ident $(+ $bounds:ident)*) => {
        impl fmt::Debug for dyn $base $(+ $bounds)* {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.pad(stringify!($base $(+ $bounds)*))
            }
        }
    }
}
impl<T: ?Sized + AsAny> Downcast for T {}

implement!(AsAny);
implement!(AsAny + Send);
implement!(AsAny + Sync);
implement!(AsAny + Send + Sync);
