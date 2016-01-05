use ::incubation::TupleIndexingExt;

pub struct Iter<T> {
    tuple: T,
    index: usize
}

pub trait TupleIterExt {
    fn iter(&self) -> Iter<&Self>;
}

impl <T> TupleIterExt for T {
    fn iter(&self) -> Iter<&Self> {
        Iter { tuple: self, index: 0 }
    }
}

macro_rules! impl_iter {
    ($T:ty) => {
        impl <'a, T> Iterator for Iter<&'a $T> {
            type Item = &'a T;
            fn next(&mut self) -> Option<Self::Item> {
                let t = self.tuple.get(self.index);
                if let Some(_) = t {
                    self.index += 1;
                }
                t
            }
        }
    }
}

impl_iter!((T,));
impl_iter!((T,T));
impl_iter!((T,T,T));
impl_iter!((T,T,T,T));
impl_iter!((T,T,T,T,T));
impl_iter!((T,T,T,T,T,T));
impl_iter!((T,T,T,T,T,T,T));
impl_iter!((T,T,T,T,T,T,T,T));
impl_iter!((T,T,T,T,T,T,T,T,T));
impl_iter!((T,T,T,T,T,T,T,T,T,T));
