/// Returns `None` if `pred` is `false`, otherwise
/// it returns `Some(result_of_calling_f)`.
///
/// # Example
/// ```
/// use monster::incubation::some_if;
/// assert_eq!(some_if(false, || 42), None    );
/// assert_eq!(some_if(true , || 42), Some(42));
/// ```
pub fn some_if<T, F: Fn() -> T>(pred: bool, f: F) -> Option<T> {
    if pred { Some(f()) }
    else { None }
}
