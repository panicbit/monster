/// Temporarily rebind variables as mutable
///
/// # Example
/// ```
/// #[macro_use(mutate)]
/// extern crate monster;
///
/// fn main() {
///     let x = 42;
///     let y = 777;
///
///     mutate!(|x, y| {
///         x += 1;
///         y += 1;
///     });
///
///     println!("{}, {}", x, y);
/// }
/// ```
#[macro_export]
macro_rules! mutate {
    (|$($var:ident),+| $code:expr) => (
        let ($($var),+,) = {
            $( let mut $var = $var; )+
            $code;
            ($($var),+,)
        };
    );
}
