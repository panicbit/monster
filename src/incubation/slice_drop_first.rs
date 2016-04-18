//!
//! # Example
//! ```
//! use monster::incubation::SliceDropFirst;
//! let foo = &[1,2,3];
//! 
//! assert_eq!(foo.drop_first(2), &[3]);
//! assert_eq!(foo.drop_first(3), &[]);
//! assert_eq!(foo.drop_first(9000), &[]);
//! ```

pub trait SliceDropFirst {
    /// Drop the first `n` elements from the slice.
    /// If `n` is bigger than the slice's length
    /// an empty slice will be returned.
    fn drop_first(self, n: usize) -> Self;
}

impl <'a, T> SliceDropFirst for &'a [T] {
    fn drop_first(self, n: usize) -> Self {
        if n > self.len() {
            &[]
        } else {
            &self[n..]
        }
    }
}
