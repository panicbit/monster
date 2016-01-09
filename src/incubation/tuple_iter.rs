// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use ::incubation::TupleIndexingExt;

pub struct Iter<T> {
    tuple: T,
    index: usize
}

pub trait TupleIterExt {
    fn iter(&self) -> Iter<&Self>;
}

impl <T> TupleIterExt for T {
    fn iter(&self) -> Iter<&Self> {
        Iter { tuple: self, index: 0 }
    }
}

macro_rules! impl_iter {
    ($T:ty) => {
        impl <'a, T> Iterator for Iter<&'a $T> {
            type Item = &'a T;
            fn next(&mut self) -> Option<Self::Item> {
                let t = self.tuple.get(self.index);
                if let Some(_) = t {
                    self.index += 1;
                }
                t
            }
        }
    }
}

impl_iter!((T,));
impl_iter!((T,T));
impl_iter!((T,T,T));
impl_iter!((T,T,T,T));
impl_iter!((T,T,T,T,T));
impl_iter!((T,T,T,T,T,T));
impl_iter!((T,T,T,T,T,T,T));
impl_iter!((T,T,T,T,T,T,T,T));
impl_iter!((T,T,T,T,T,T,T,T,T));
impl_iter!((T,T,T,T,T,T,T,T,T,T));
