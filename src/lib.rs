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

use core::{any::Any as StdAny, fmt};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

pub trait Any: StdAny {}

impl<T: StdAny> Any for T {}

// source: https://github.com/chris-morgan/anymap/blob/master/src/any.rs
/// This trait is an extension trait to [`Any`], and adds methods for unchecked downcasts
pub trait UncheckedAnyExt: Any {
    unsafe fn downcast_ref_unchecked<T: Any>(&self) -> &T;
    unsafe fn downcast_mut_unchecked<T: Any>(&mut self) -> &mut T;
    #[cfg(feature = "alloc")]
    unsafe fn downcast_unchecked<T: Any>(self: Box<Self>) -> Box<T>;
}

#[cfg(feature = "alloc")]
/// A trait for the conversion of an object into a boxed trait object.
pub trait IntoBox<A: ?Sized + UncheckedAnyExt>: Any {
    /// Convert self into the appropriate boxed form.
    fn into_box(self) -> Box<A>;
}

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn StdAny;
    fn as_any_mut(&mut self) -> &mut dyn StdAny;

    #[doc(hidden)]
    fn as_my_any(&self) -> &dyn Any;

    #[doc(hidden)]
    fn as_my_any_mut(&mut self) -> &mut dyn Any;

    /// Gets the type name of `self`
    fn type_name(&self) -> &'static str;
}

impl<T: StdAny> AsAny for T {
    #[inline(always)]
    fn as_any(&self) -> &dyn StdAny {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn StdAny {
        self
    }

    #[doc(hidden)]
    #[inline(always)]
    fn as_my_any(&self) -> &dyn Any {
        self
    }

    #[doc(hidden)]
    #[inline(always)]
    fn as_my_any_mut(&mut self) -> &mut dyn Any {
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
        self.as_my_any().downcast_ref_unchecked()
    }

    #[inline]
    unsafe fn downcast_mut_unchecked<T: Any>(&mut self) -> &mut T {
        self.as_my_any_mut().downcast_mut_unchecked()
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
    }
}

implement!(Any);
implement!(Any + Send);
implement!(Any + Sync);
implement!(Any + Send + Sync);

implement!(AsAny);
implement!(AsAny + Send);
implement!(AsAny + Sync);
implement!(AsAny + Send + Sync);

impl Downcast for dyn AsAny {}
impl Downcast for dyn AsAny + Send {}
impl Downcast for dyn AsAny + Sync {}
impl Downcast for dyn AsAny + Send + Sync {}
