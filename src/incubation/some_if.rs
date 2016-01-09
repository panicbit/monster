// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

/// Returns `None` if `pred` is `false`, otherwise
/// it returns `Some(result_of_calling_f)`.
///
/// # Example
/// ```
/// use monster::incubation::some_if;
/// assert_eq!(some_if(false, || 42), None    );
/// assert_eq!(some_if(true , || 42), Some(42));
/// ```
pub fn some_if<T, F: Fn() -> T>(pred: bool, f: F) -> Option<T> {
    if pred { Some(f()) }
    else { None }
}
