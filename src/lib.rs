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

// implements the downcasting methods
impl as_any::Downcast for dyn Custom {}
impl as_any::Downcast for dyn Custom + Send {}
impl as_any::Downcast for dyn Custom + Sync {}
impl as_any::Downcast for dyn Custom + Send + Sync {}

impl Custom for Test {}

fn lol() {
    let x = Test;
    let y: &dyn Custom = &x;
    y.downcast_ref::<Test>().unwrap();
}
```
**/

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::{any::Any, fmt};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

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

// source: https://github.com/chris-morgan/anymap/blob/master/src/any.rs
/// This trait is an extension trait to [`AsAny`], and adds methods for unchecked downcasts
pub trait UncheckedAnyExt: AsAny {
    unsafe fn downcast_ref_unchecked<T: AsAny>(&self) -> &T;
    unsafe fn downcast_mut_unchecked<T: AsAny>(&mut self) -> &mut T;
    #[cfg(feature = "alloc")]
    unsafe fn downcast_unchecked<T: AsAny>(self: Box<Self>) -> Box<T>;
}

#[cfg(feature = "alloc")]
/// A trait for the conversion of an object into a boxed trait object.
pub trait IntoBox<A: ?Sized + UncheckedAnyExt>: AsAny {
    /// Convert self into the appropriate boxed form.
    fn into_box(self) -> Box<A>;
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

    #[inline]
    unsafe fn downcast_ref_unchecked<T: Any>(&self) -> &T {
        UncheckedAnyExt::downcast_ref_unchecked(self.as_my_any())
    }

    #[inline]
    unsafe fn downcast_mut_unchecked<T: Any>(&mut self) -> &mut T {
        UncheckedAnyExt::downcast_mut_unchecked(self.as_my_any_mut())
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

        impl UncheckedAnyExt for dyn $base $(+ $bounds)* {
            #[inline]
            unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
                &*(self as *const Self as *const T)
            }

            #[inline]
            unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
                &mut *(self as *mut Self as *mut T)
            }

            #[cfg(feature = "alloc")]
            #[inline]
            unsafe fn downcast_unchecked<T: 'static>(self: Box<Self>) -> Box<T> {
                Box::from_raw(Box::into_raw(self) as *mut T)
            }
        }

        #[cfg(feature = "alloc")]
        impl<T: $base $(+ $bounds)*> IntoBox<dyn $base $(+ $bounds)*> for T {
            #[inline]
            fn into_box(self) -> Box<dyn $base $(+ $bounds)*> {
                Box::new(self)
            }
        }

        impl Downcast for dyn $base $(+ $bounds)* {}
    }
}

implement!(AsAny);
implement!(AsAny + Send);
implement!(AsAny + Sync);
implement!(AsAny + Send + Sync);
