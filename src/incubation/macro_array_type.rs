/// Create an array wrapper that implements
/// Clone, PartialEq, Eq and Debug
///
/// # Example
///
/// ```
/// #[macro_use(array_type)]
/// extern crate monster;
///
/// array_type!(Bytes4, u8, 4);
///
/// fn main() {
///     let key = Bytes4([1,2,3,4]);
///     let key2 = key.clone();
///     assert_eq!(key, key2);
/// }
/// ```
#[macro_export]
macro_rules! array_type {
    ($NAME:ident, $TYP:ty, $SIZE:expr) => {
        pub struct $NAME(pub [$TYP; $SIZE]);
        
        impl ::std::clone::Clone for $NAME {
            fn clone(&self) -> $NAME {
                unsafe {
                    let mut copy: [$TYP; $SIZE] = ::std::mem::uninitialized();
                    copy.clone_from_slice(&self.0);
                    $NAME(copy)
                }
            }
        }
        
        impl<'a> ::std::cmp::PartialEq<$NAME> for $NAME {
            fn eq(&self, other: &$NAME) -> bool {
                &self.0 == &other.0
            }
        }
        
        impl ::std::cmp::PartialEq<[$TYP]> for $NAME {
            fn eq(&self, other: &[$TYP]) -> bool {
                &self.0 == other
            }
        }
        
        impl ::std::cmp::PartialEq<$NAME> for [$TYP] {
            fn eq(&self, other: &$NAME) -> bool {
                other == self
            }
        }
        
        impl ::std::cmp::Eq for $NAME {}
        
        impl ::std::ops::Deref for $NAME {
            type Target = [$TYP];
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl ::std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                (&self.0).fmt(f)
            }
        }
    }
}
