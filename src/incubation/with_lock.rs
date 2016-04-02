use std::sync::Mutex;

pub trait WithLock<T> {
    /// Conveniently access mutexes.
    ///
    /// # Example
    /// ```
    /// use std::sync::Mutex;
    /// use monster::incubation::WithLock;
    ///
    /// let n = Mutex::new(123);
    ///
    /// n.with_lock(|n| {
    ///     *n += 654
    /// });
    /// ```
    fn with_lock<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R;
}

impl <T> WithLock<T> for Mutex<T> {
    fn with_lock<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R {
        let ref mut data = self.lock().unwrap();
        f(data)
    }
}
