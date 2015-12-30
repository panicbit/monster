macro_rules! impl_struct_flip {
    ($struct_name:ident, $($TN:ident),*) => (
        pub struct $struct_name<F, T> {
            f: F,
            _marker: ::std::marker::PhantomData<T>
        }

        impl <F, T> $struct_name<F, T> {
            fn new(f: F) -> $struct_name<F, T> {
                $struct_name {
                    f: f,
                    _marker: ::std::marker::PhantomData
                }
            }
        }

        impl <F, R, T1, T2, $($TN),*> FnOnce<(T2, T1, $($TN),*)> for $struct_name<F, (T1, T2, $($TN),*)> where
            F: FnOnce(T1, T2, $($TN),*) -> R
        {
            type Output = R;
            extern "rust-call" fn call_once(self, args: (T2, T1, $($TN),*)) -> Self::Output {
                #[allow(non_snake_case)]
                let (T2, T1, $($TN),*) = args;

                (self.f)(T1, T2, $($TN),*)
            }
        }
    )
}

impl_struct_flip!(Flip,);
impl_struct_flip!(Flip3, T3);
impl_struct_flip!(Flip4, T3, T4);
impl_struct_flip!(Flip5, T3, T4, T5);
impl_struct_flip!(Flip6, T3, T4, T5, T6);
impl_struct_flip!(Flip7, T3, T4, T5, T6, T7);
impl_struct_flip!(Flip8, T3, T4, T5, T6, T7, T8);
impl_struct_flip!(Flip9, T3, T4, T5, T6, T7, T8, T9);
impl_struct_flip!(Flip10, T3, T4, T5, T6, T7, T8, T9, T10);

pub mod traits {  
    macro_rules! impl_trait_flip_ext {
        ($struct_name:ident, $trait_name:ident, $meth_name:ident, $($TN:ident),*) => (
            pub trait $trait_name<T>: Sized {
                fn $meth_name(self) -> super::$struct_name<Self, T> {
                    super::$struct_name::new(self)
                }
            }

            impl <F, R, T1, T2, $($TN),*> $trait_name<(T1, T2, $($TN),*)> for F where
                F: FnOnce(T1, T2, $($TN),*) -> R
            {}
        )
    }
    impl_trait_flip_ext!(Flip, FlipFnExt, flip,);
    impl_trait_flip_ext!(Flip3, Flip3FnExt, flip3, T3);
    impl_trait_flip_ext!(Flip4, Flip4FnExt, flip4, T3, T4);
    impl_trait_flip_ext!(Flip5, Flip5FnExt, flip5, T3, T4, T5);
    impl_trait_flip_ext!(Flip6, Flip6FnExt, flip6, T3, T4, T5, T6);
    impl_trait_flip_ext!(Flip7, Flip7FnExt, flip7, T3, T4, T5, T6, T7);
    impl_trait_flip_ext!(Flip8, Flip8FnExt, flip8, T3, T4, T5, T6, T7, T8);
    impl_trait_flip_ext!(Flip9, Flip9FnExt, flip9, T3, T4, T5, T6, T7, T8, T9);
    impl_trait_flip_ext!(Flip10, Flip10FnExt, flip10, T3, T4, T5, T6, T7, T8, T9, T10);
}

pub use self::traits::*;

pub mod methods {
    macro_rules! impl_fn_flip {
        ($struct_name:ident, $meth_name:ident, $($TN:ident),*) => (
            pub fn $meth_name<F, R, T1, T2, $($TN),*>(f: F) -> super::$struct_name<F,(T1, T2, $($TN),*)> where
                F: FnOnce(T1, T2, $($TN),*) -> R
            {
                super::$struct_name::new(f)
            }
        )
    }
    impl_fn_flip!(Flip, flip,);
    impl_fn_flip!(Flip3, flip3, T3);
    impl_fn_flip!(Flip4, flip4, T3, T4);
    impl_fn_flip!(Flip5, flip5, T3, T4, T5);
    impl_fn_flip!(Flip6, flip6, T3, T4, T5, T6);
    impl_fn_flip!(Flip7, flip7, T3, T4, T5, T6, T7);
    impl_fn_flip!(Flip8, flip8, T3, T4, T5, T6, T7, T8);
    impl_fn_flip!(Flip9, flip9, T3, T4, T5, T6, T7, T8, T9);
    impl_fn_flip!(Flip10, flip10, T3, T4, T5, T6, T7, T8, T9, T10);
}

pub use self::methods::*;

#[cfg(test)]
mod test {
    use super::traits::*;
    use super::methods::*;

    fn demo_2(foo: &str, bar: &str) -> String { format!("{}, {}", foo, bar) }
    fn demo_3(foo: &str, bar: &str, quux: &str) -> String { format!("{}, {}, {}", foo, bar, quux) }

    #[test] 
    fn flip_trait() {
        assert_eq!(demo_2.flip()("bar", "foo"), "foo, bar");
        assert_eq!(demo_3.flip3()("bar", "foo", "quux"), "foo, bar, quux");
    }

    #[test] 
    fn flip_methods() {
        assert_eq!(flip(demo_2)("bar", "foo"), "foo, bar");
        assert_eq!(flip3(demo_3)("bar", "foo", "quux"), "foo, bar, quux");
    }
}
