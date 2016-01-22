//!
//! # Example
//!
//! ```
//! use monster::incubation::FindAndTake;
//!
//! let mut v = vec!["foo", "bar", "quux"];
//!
//! let elem = v.find_and_take(|&s| s == "bar");
//!
//! assert_eq!(elem, Some("bar"));
//! assert_eq!(v, &["foo", "quux"]);
//! ```

pub trait FindAndTake {
    type Item;
    fn find_and_take<F: Fn(&Self::Item) -> bool>(&mut self, f: F) -> Option<Self::Item>;
}

impl <T> FindAndTake for Vec<T> {
    type Item = T;
    fn find_and_take<F: Fn(&Self::Item) -> bool>(&mut self, f: F) -> Option<Self::Item> {
        let index = self
            .iter()
            .enumerate()
            .find(|&(_, ref item)| f(item))
            .map(|(i, _)| i);

        index.map(|index| self.remove(index))
    }
}

impl <T> FindAndTake for Option<T> {
    type Item = T;
    fn find_and_take<F: Fn(&Self::Item) -> bool>(&mut self, f: F) -> Option<Self::Item> {
        if self.as_ref().map(f).unwrap_or(false) {
            self.take()
        } else {
            None
        }
    }
}
