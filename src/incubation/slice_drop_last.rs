//!
//! # Example
//! ```
//! use monster::incubation::SliceDropLast;
//! let foo = &[1,2,3];
//! 
//! assert_eq!(foo.drop_last(2), &[1]);
//! assert_eq!(foo.drop_last(3), &[]);
//! assert_eq!(foo.drop_last(9000), &[]);
//! ```

pub trait SliceDropLast {
    /// Drop the last `n` elements from the slice.
    /// If `n` is bigger than the slice's length
    /// an empty slice will be returned.
    fn drop_last(self, n: usize) -> Self;
}

impl <'a, T> SliceDropLast for &'a [T] {
    fn drop_last(self, n: usize) -> Self {
        if n > self.len() {
            &[]
        } else {
            &self[..self.len()-n]
        }
    }
}
