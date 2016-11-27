use std::ops::{Deref, DerefMut};

/// `OwningRefMut` allows you to return mutably borrowed values.
/// Careful usage is advised, the lifetime semantics might not
/// be entirely sound.
///
/// # Example
/// 
/// ```
/// use monster::incubation::OwningRefMut;
///
/// fn demo() -> OwningRefMut<Vec<i32>, Box<Iterator<Item=i32>>> {
///     let elems = vec![1,2,3];
///     OwningRefMut::new(Box::new(elems), |elems|
///         Box::new(
///             elems.iter().map(|n| n*2)
///         ) as Box<_>
///     )
/// }
/// 
/// let items: Vec<_> = demo().collect();
/// assert_eq!(items, [2, 4, 6]);
/// ```

pub struct OwningRefMut<T, R> {
    owned: *mut T,
    borrow: Option<R>
}

impl <'a, T: 'a, R> OwningRefMut<T, R> {
    pub fn new<F: FnOnce(&'a mut T) -> R>(owned: Box<T>, f: F) -> OwningRefMut<T, R> {
        unsafe {
            let owned = Box::into_raw(owned);
            let borrow = f(&mut *owned);
            OwningRefMut {
                owned: owned,
                borrow: Some(borrow)
            }
        }
    }
}

impl <T, R> Drop for OwningRefMut<T, R> {
    fn drop(&mut self) {
        // Drop borrow first, then drop owned
        self.borrow.take();
        unsafe { Box::from_raw(self.owned) };
    }
}

impl <T, R> Deref for OwningRefMut<T, R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        self.borrow.as_ref().expect("bug")
    }
}

impl <T, R> DerefMut for OwningRefMut<T, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow.as_mut().unwrap()
    }
}

impl <T, R: Iterator<Item=U>, U> Iterator for OwningRefMut<T, R> {
    type Item = U;
    fn next(&mut self) -> Option<Self::Item> {
        self.deref_mut().next()
    }
}
