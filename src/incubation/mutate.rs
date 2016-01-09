// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

/// Temporarily rebind variables as mutable
///
/// # Example
/// ```
/// #[macro_use(mutate)]
/// extern crate monster;
///
/// fn main() {
///     let x = 42;
///     let y = 777;
///
///     mutate!(|x, y| {
///         x += 1;
///         y += 1;
///     });
///
///     assert_eq!((x, y), (43, 778));
/// }
/// ```
#[macro_export]
macro_rules! mutate {
    (|$($var:ident),+| $code:expr) => (
        let ($($var),+,) = {
            $( let mut $var = $var; )+
            $code;
            ($($var),+,)
        };
    );
}
