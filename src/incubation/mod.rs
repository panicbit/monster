//! This module includes new utilities
//! which are being matured and tested for permanent inclusion.
//! Items in this module should be considered unstable and thus
//! may change or get removed completely at any time.

pub mod swap_tuple;
pub use self::swap_tuple::SwapTupleExt;

pub mod mutate;

pub mod map_ref_mut;
pub use self::map_ref_mut::{
    MapRefMutExt,
    map_ref_mut
};

pub mod anon;

pub mod zip_ends;
pub use self::zip_ends::{
    ZipEnds,
    ZipEndsIterExt
};

pub mod fmt;
pub use self::fmt::{
    format,
    write_format
};

#[cfg(feature="unstable")]
pub mod flip;
#[cfg(feature="unstable")]
pub use self::flip::methods::{flip, flip3};
#[cfg(feature="unstable")]
pub use self::flip::traits::{FlipFnExt, Flip3FnExt};

pub mod some_if;
pub use self::some_if::some_if;

pub mod tuple_indexing;
pub use self::tuple_indexing::TupleIndexingExt;

pub mod tuple_iter;
pub use self::tuple_iter::TupleIterExt;

pub mod ref_eq;
pub use self::ref_eq::RefEq;

pub mod global_init_cleanup;
pub use self::global_init_cleanup::{InitCleanup, GlobalInitHandle, InitHandle};

pub mod with_lock;
pub use self::with_lock::WithLock;

pub mod find_and_take;
pub use self::find_and_take::FindAndTake;

pub mod slice_drop_last;
pub use self::slice_drop_last::SliceDropLast;

pub mod slice_take_while;
pub use self::slice_take_while::SliceTakeWhile;

pub mod split_int;
pub use self::split_int::{SplitInt, split_mut_u16};

pub mod reverse_bitorder;
pub use self::reverse_bitorder::{ReverseBitorder, reverse_bitorder};
