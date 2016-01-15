use std::sync::Mutex;

pub trait WithLock<T> {
    fn with_lock<F: Fn(&mut T) -> R, R>(&self, f: F) -> R;
}

impl <T> WithLock<T> for Mutex<T> {
    fn with_lock<F: Fn(&mut T) -> R, R>(&self, f: F) -> R {
        let ref mut data = self.lock().unwrap();
        f(data)
    }
}
