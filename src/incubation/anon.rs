/// Create an anonymous struct.
///
/// # Example
/// ```
/// #[macro_use(anon)]
/// extern crate monster;
///
/// fn main() {
///     let foo = anon!(Data {
///         field_a: i32 = 0,
///         field_b: bool = true
///     });
///     
///     println!("{:?}", foo);
/// }
/// ```
#[macro_export]
macro_rules! anon {
    ($($field:ident : $typ:ty = $value:expr),+) => ({
        anon!(Anon {
            $( $field: $typ = $value ),*
        })
    });
    ($struct_name:ident { $($field:ident : $typ:ty = $value:expr),+ }) => ({
        #[derive(Debug)]
        struct $struct_name {
            $( $field: $typ ),*
        }
        
        $struct_name {
            $( $field: $value ),*
        }
    });
}
