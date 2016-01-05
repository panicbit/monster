
pub trait TupleIndexingExt<T> {
    fn get(&self, n: usize) -> Option<&T>;
}

impl <T> TupleIndexingExt<T> for () {
    fn get(&self, _: usize) -> Option<&T> {
        None
    }
}

impl <T> TupleIndexingExt<T> for (T,) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T,T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T,T,T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T,T,T,T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T,T,T,T,T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T,T,T,T,T,T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T,T,T,T,T,T,T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            _ => None
        }
    }
}

impl <T> TupleIndexingExt<T> for (T,T,T,T,T,T,T,T,T,T) {
    fn get(&self, n: usize) -> Option<&T> {
        match n {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            _ => None
        }
    }
}