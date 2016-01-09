// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::iter::*;

/// An iterator adaptor to iterate over pairs of starting and ending elements.
///
/// # Example
/// ```
/// use monster::incubation::ZipEndsIterExt;
/// 
/// fn is_palindrome<T: Eq + PartialEq>(slice: &[T]) -> bool {
///     slice.zip_ends().all(|(a, b)| a == b)
/// }
/// ```
pub struct ZipEnds<I> {
    iter: I
}

impl <'a, T, I> Iterator for ZipEnds<I> where
    T: 'a,
    I: Iterator<Item=&'a T> + DoubleEndedIterator
{
    type Item = (&'a T, &'a T);
    
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next();
        let b = self.iter.next_back();
        
        match (a, b) {
            (Some(a), Some(b)) => Some((a, b)),
            (Some(a), _      ) => Some((a, a)),
            (_      , Some(b)) => Some((b, b)),
            (None   , None   ) => None
        }
    }
}

pub trait ZipEndsIterExt: Sized {
    type Iter;
    /// Create a new `ZipEnds` adaptor.
    /// See the structs' documentation for more.
    fn zip_ends(self) -> ZipEnds<Self::Iter>;
}

impl <'a, T, I, U> ZipEndsIterExt for U where
    T: 'a,
    I: Iterator<Item=&'a T> + DoubleEndedIterator,
    U: IntoIterator<Item=I::Item, IntoIter=I>
{
    type Iter = I;
    fn zip_ends(self) -> ZipEnds<Self::Iter> {
        ZipEnds {
            iter: self.into_iter()
        }
    }
}