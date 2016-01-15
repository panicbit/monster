macro_rules! e { ($e:expr) => {$e} }

macro_rules! match_index {
    ($n:expr, $this:expr, $($i:tt),*) => (
        match $n {
            $(
                e!($i) => Some(&e!($this . $i)),
            )*
            _ => None
        }
    )
}

macro_rules! impl_indexing {
    ($typ:ty, $($i:tt),*) => (
        impl <T> TupleIndexingExt<T> for $typ {
            fn get(&self, n: usize) -> Option<&T> {
                match_index!(n, self, $($i),*)
            }
        }
    )
}

pub trait TupleIndexingExt<T> {
    fn get(&self, n: usize) -> Option<&T>;
}

impl <T> TupleIndexingExt<T> for () {
    fn get(&self, _: usize) -> Option<&T> {
        None
    }
}

impl_indexing!(
    (T,),
    0
);

impl_indexing!(
    (T, T),
    0, 1
);

impl_indexing!(
    (T, T, T),
    0, 1, 2
);

impl_indexing!(
    (T, T, T, T),
    0, 1, 2, 3
);

impl_indexing!(
    (T, T, T, T, T),
    0, 1, 2, 3, 4
);

impl_indexing!(
    (T, T, T, T, T, T),
    0, 1, 2, 3, 4, 5
);

impl_indexing!(
    (T, T, T, T, T, T, T),
    0, 1, 2, 3, 4, 5, 6
);

impl_indexing!(
    (T, T, T, T, T, T, T, T),
    0, 1, 2, 3, 4, 5, 6, 7
);

impl_indexing!(
    (T, T, T, T, T, T, T, T, T),
    0, 1, 2, 3, 4, 5, 6, 7, 8
);

impl_indexing!(
    (T, T, T, T, T, T, T, T, T, T),
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9
);
