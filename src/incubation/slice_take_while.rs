
pub trait SliceTakeWhile {
    type Item;
    fn take_while<F>(self, f: F) -> Self where
        F: Fn(&Self::Item) -> bool;
}

impl <'a, T> SliceTakeWhile for &'a [T] {
    type Item = T;
    fn take_while<F>(self, f: F) -> Self where
        F: Fn(&Self::Item) -> bool
    {
        slice_take_while(self, f)
    }
}

pub fn slice_take_while<T, F>(slice: &[T], f: F) -> &[T] where
    F: Fn(&T) -> bool
{
    for i in 0 .. slice.len() {
        let elem = unsafe { slice.get_unchecked(i) };
        if !f(elem) {
            return &slice[..i];
        }
    }

    slice
}

#[cfg(test)]
mod test {
    use super::SliceTakeWhile;
    #[test]
    fn slice_take_while_basic() {
        let data = &[1,2,3,4];
        let result = data.take_while(|el| *el <= 2);

        assert_eq!(result, &[1,2]);
    }

    fn slice_take_while_empty() {
        let data = &[1,2,3,4];
        let result = data.take_while(|_| false);

        assert_eq!(result, &[]);
    }

    fn slice_take_while_identity() {
        let data = &[1,2,3,4];
        let result = data.take_while(|_| true);

        assert_eq!(result, data);
    }
}