use super::AbsDiffEq;

#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
macro_rules! impl_abs_diff_eq {
    () => {
        impl AbsDiffEq for () {
            type Epsilon = ();

            fn default_epsilon() -> Self::Epsilon {}

            fn abs_diff_eq(&self, _other: &Self, _epsilon: Self::Epsilon) -> bool {
                true
            }
        }
    };

    ($($idx:tt),+) => {
        paste::paste! {
            impl<$( [<T $idx>], )+> AbsDiffEq for ($( [<T $idx>], )+)
            where
                $( [<T $idx>]: AbsDiffEq, )+
            {
                type Epsilon = ($( [<T $idx>]::Epsilon, )+);

                fn default_epsilon() -> Self::Epsilon {
                    ($( [<T $idx>]::default_epsilon(), )+)
                }

                fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                    true $( && self.$idx.abs_diff_eq(&other.$idx, epsilon.$idx) )+
                }
            }
        }
    };
}

#[cfg(feature = "tuple_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "tuple_impl")))]
mod abs_diff_eq_tuple_impls {
    use super::*;

    impl_abs_diff_eq!();
    impl_abs_diff_eq!(0);
    impl_abs_diff_eq!(0, 1);
    impl_abs_diff_eq!(0, 1, 2);
    impl_abs_diff_eq!(0, 1, 2, 3);
    impl_abs_diff_eq!(0, 1, 2, 3, 4);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    impl_abs_diff_eq!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
}
