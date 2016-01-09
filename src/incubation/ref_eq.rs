// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::ops::Deref;

pub trait RefEq: Deref {
    fn ref_eq<T: Deref<Target = Self::Target>>(&self, other: &T) -> bool {
        &**self as *const <Self as Deref>::Target ==
        &**other as *const <Self as Deref>::Target
    }
}

impl <T: Deref> RefEq for T {}

#[cfg(test)]
mod test {
    use super::RefEq;

    #[test]
    fn ref_eq_literal() {
        let a = &42;
        let b = &42;
        let c = a;

        assert!(a.ref_eq(&b));
        assert!(b.ref_eq(&c));
        assert!(a.ref_eq(&c));
    }

    #[test]
    fn ref_eq_box() {
        let box_a = Box::new(42);
        let box_b = Box::new(42);
        let box_c = &*box_a;
        
        assert!(!box_a.ref_eq(&box_b));
        assert!(!box_b.ref_eq(&box_c));
        assert!(box_a.ref_eq(&box_c));
    }


    #[test]
    fn ref_eq_rc() {
        use std::rc::Rc;
        
        let rc_a = Rc::new(42);
        let rc_b = Rc::new(42);
        let rc_c = rc_a.clone();
        
        assert!(!rc_a.ref_eq(&rc_b));
        assert!(!rc_b.ref_eq(&rc_c));
        assert!(rc_a.ref_eq(&rc_c));
    }
}
