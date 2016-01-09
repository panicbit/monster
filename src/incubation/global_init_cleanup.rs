// Copyright (c) 2015 monster developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//!
//! # Example
//! ```
//! #[macro_use]
//! extern crate lazy_static;
//! extern crate monster;
//! 
//! use monster::incubation::global_init_cleanup::*;
//! 
//! lazy_static! {
//!     static ref INIT_HANDLE: GlobalInitHandle<MyLib> = InitHandle::new_global();
//! }
//! 
//! struct MyLib {
//!     init_handle: LocalInitHandle<MyLib>
//! }
//! 
//! impl MyLib {
//!     fn new() -> MyLib {
//!         let init_handle = InitHandle::from_global(&INIT_HANDLE);
//! 
//!         MyLib {
//!             init_handle: init_handle
//!         }
//!     }
//! }
//! 
//! impl InitCleanup for MyLib {
//!     fn init() { println!("LIB_INIT()"); }
//!     fn cleanup() { println!("LIB_CLEANUP()"); }
//! }
//! 
//! fn main() {
//!     let l1 = MyLib::new();
//!     let l2 = MyLib::new();
//!     let l3 = MyLib::new();
//! }
//! ```

use std::sync::{Arc,Weak,Mutex};
use std::marker;

pub trait InitCleanup {
    fn init();
    fn cleanup();
}

pub struct InitHandle<T: InitCleanup>(marker::PhantomData<T>);

impl <T: InitCleanup> InitHandle<T> {
    pub fn new_global() -> GlobalInitHandle<T> {
        Mutex::new(None)
    }

    pub fn from_global(handle: &Mutex<Option<Weak<InitHandle<T>>>>) -> LocalInitHandle<T> {
        let mut handle = handle.lock().unwrap();
        handle.as_ref()
        .and_then(Weak::upgrade)
        .unwrap_or_else(|| {
            let new_handle = Arc::new(InitHandle(marker::PhantomData));
            *handle = Some(Arc::downgrade(&new_handle));

            T::init();

            new_handle
        })
    }
}

impl <T: InitCleanup> Drop for InitHandle<T> {
    fn drop(&mut self) {
        T::cleanup()
    }
}


pub type GlobalInitHandle<T> = Mutex<Option<Weak<InitHandle<T>>>>;
pub type LocalInitHandle<T> = Arc<InitHandle<T>>;
