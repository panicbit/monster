/// Implicitly initialize a set of tests
///
/// # Example
///
/// ```
/// #[macro_use(init_tests)]
/// extern crate monster;
///
/// use std::sync::{Once, ONCE_INIT};
/// 
/// static LOG_INIT: Once = ONCE_INIT;
///
/// init_tests! {
///     init => {
///         println!("init call in every test");
///         LOG_INIT.call_once(|| println!("initializing logger once"));
///     }
/// 
///     fn foo() {
///         panic!("foo");
///     }
///     
///     fn bar() {
///         panic!("bar");
///     }
/// }
///
///# fn main() {}
/// ```
#[macro_export]
macro_rules! init_tests {
    {
    init => $init:block
    $(fn $fn_name:ident($($arg_name:ident : $arg_ty:ty),*) { $($code:stmt);*$(;)* })*} => (
        $(
            #[test]
            fn $fn_name($($arg_name: $arg_ty),*) {
                $init
                $($code);*;
            }
        )*
    )
}
