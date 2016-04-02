use std::mem;

/// Map the value of a mutable reference.
/// Useful if you want to apply a `FnOnce(T) -> T` to a `&mut T`.
/// This function is unsafe because panicking in `f`
/// would leave the pointee uninitialized.
///
/// # Example
/// ```
/// use monster::incubation::map_ref_mut;
///
/// let foo = &mut 123;
///
/// unsafe { map_ref_mut(foo, |n| n + 654) }
///
/// assert_eq!(*foo, 777);
/// ```
pub unsafe fn map_ref_mut<T, F>(thing: &mut T, f: F) where
    F: FnOnce(T) -> T
{
    let dummy: T = mem::uninitialized();
    let owned_thing = mem::replace(thing, dummy);
    let new_thing = f(owned_thing);
    let dummy = mem::replace(thing, new_thing);
    mem::forget(dummy);
}

pub trait MapRefMutExt: Sized {
    /// Map the value of a mutable reference.
    /// Useful if you want to apply a `FnOnce(T) -> T` to a `&mut T`.
    /// This function is unsafe because panicking in `f`
    /// would leave the pointee uninitialized.
    ///
    /// # Example
    /// ```
    /// use monster::incubation::MapRefMutExt;
    ///
    /// let mut foo = &mut 123;
    ///
    /// unsafe { foo.map_ref_mut(|n| n + 654) }
    ///
    /// assert_eq!(*foo, 777);
    /// ```
    unsafe fn map_ref_mut<F>(&mut self, f: F) where
        F: FnOnce(Self) -> Self
    {
        map_ref_mut(self, f)
    }
}

impl <T> MapRefMutExt for T {}
