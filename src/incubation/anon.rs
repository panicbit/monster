// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

/// Create an anonymous struct.
///
/// # Example
/// ```
/// #[macro_use(anon)]
/// extern crate monster;
///
/// fn main() {
///     let foo = anon!(Data {
///         field_a: i32 = 0,
///         field_b: bool = true
///     });
///     
///     println!("{:?}", foo);
/// }
/// ```
#[macro_export]
macro_rules! anon {
    ($($field:ident : $typ:ty = $value:expr),+) => ({
        anon!(Anon {
            $( $field: $typ = $value ),*
        })
    });
    ($struct_name:ident { $($field:ident : $typ:ty = $value:expr),+ }) => ({
        #[derive(Debug)]
        struct $struct_name {
            $( $field: $typ ),*
        }
        
        $struct_name {
            $( $field: $value ),*
        }
    });
}
