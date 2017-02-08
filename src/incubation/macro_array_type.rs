/// Create an array wrapper which implements
///
/// - `Copy`
/// - `Clone`
/// - `PartialEq`
/// - `Eq`
/// - `PartialOrd`
/// - `Ord`
/// - `Debug`
/// - `Display`
/// - `Deref`
/// - `AsMut`
/// - `Index`
///
/// <!-- TODO: Implement Hash -->
///
/// # Example
///
/// ```
/// #[macro_use(array_type)]
/// extern crate monster;
///
/// array_type!(Array4, 4);
///
/// fn main() {
///     let key = Array4([1,2,3,4]);
///     let key2 = key.clone();
///     assert_eq!(key, key2);
/// }
/// ```
#[macro_export]
macro_rules! array_type {
    ($NAME:ident, $SIZE:expr) => {
        pub struct $NAME<T>(pub [T; $SIZE]);

        impl<T: ::std::marker::Copy> ::std::marker::Copy for $NAME<T> {}

        impl<T: Clone> ::std::clone::Clone for $NAME<T> {
            fn clone(&self) -> Self {
                unsafe {
                    let mut copy: [T; $SIZE] = ::std::mem::uninitialized();
                    copy.clone_from_slice(&self.0);
                    $NAME(copy)
                }
            }
        }

        impl<'a, T: PartialEq<T>> ::std::cmp::PartialEq<$NAME<T>> for $NAME<T> {
            fn eq(&self, other: &Self) -> bool {
                &self.0[..] == &other.0
            }
        }

        impl<T: ::std::cmp::PartialEq<T>> ::std::cmp::PartialEq<[T]> for $NAME<T> {
            fn eq(&self, other: &[T]) -> bool {
                &self.0[..] == other
            }
        }

        impl<T: ::std::cmp::Eq> ::std::cmp::Eq for $NAME<T> {}

        impl<T: ::std::cmp::PartialOrd<T>> ::std::cmp::PartialOrd<$NAME<T>> for $NAME<T> {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                self.0[..].partial_cmp(&other.0[..])
            }
        }

        impl<T: ::std::cmp::PartialOrd<T>> ::std::cmp::PartialOrd<[T]> for $NAME<T> {
            fn partial_cmp(&self, other: &[T]) -> ::std::option::Option<::std::cmp::Ordering> {
                self.0[..].partial_cmp(other)
            }
        }

        impl<T: ::std::cmp::Ord> ::std::cmp::Ord for $NAME<T> {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.0[..].cmp(&other.0[..])
            }
        }

        impl<T> ::std::ops::Deref for $NAME<T> {
            type Target = [T];
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T: ::std::fmt::Debug> ::std::fmt::Debug for $NAME<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0[..].fmt(f)
            }
        }

        impl<T> ::std::convert::AsMut<[T]> for $NAME<T> {
            fn as_mut(&mut self) -> &mut [T] {
                &mut self.0
            }
        }

        impl<T> ::std::ops::Index<usize> for $NAME<T> {
            type Output = T;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::Range<usize>> for $NAME<T> {
            type Output = [T];
            fn index(&self, index: ::std::ops::Range<usize>) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::RangeTo<usize>> for $NAME<T> {
            type Output = [T];
            fn index(&self, index: ::std::ops::RangeTo<usize>) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::RangeFrom<usize>> for $NAME<T> {
            type Output = [T];
            fn index(&self, index: ::std::ops::RangeFrom<usize>) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::RangeFull> for $NAME<T> {
            type Output = [T];
            fn index(&self, index: ::std::ops::RangeFull) -> &Self::Output {
                &self.0[index]
            }
        }

    }
}
