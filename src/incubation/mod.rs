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
