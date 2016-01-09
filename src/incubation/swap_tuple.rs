// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

pub trait SwapTupleExt {
    type Result;
    /// Swap the two tuple elements
    fn swap(self) -> Self::Result;
}

impl <T, U> SwapTupleExt for (T, U) {
    type Result = (U, T);
    fn swap(self) -> Self::Result {
        (self.1, self.0)
    }
}

impl <'a, T, U> SwapTupleExt for &'a (T, U) {
    type Result = (&'a U, &'a T);
    fn swap(self) -> Self::Result {
        (&self.1, &self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::SwapTupleExt;

    #[test]
    fn swap_tuple_value() {
        let tuple: (&str, i32) = ("foo", 42);
        let swapped: (i32, &str) = tuple.swap();
        assert_eq!(swapped, (42, "foo"));
    }

    #[test]
    fn swap_tuple_ref() {
        let tuple: &(&str, i32) = &("foo", 42);
        let swapped: (&i32, &&str) = tuple.swap();
        assert_eq!(swapped, (&42, &"foo"));
    }
}
