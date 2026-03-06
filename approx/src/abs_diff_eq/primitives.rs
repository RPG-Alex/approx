use super::AbsDiffEq;

macro_rules! impl_unsigned_abs_diff_eq {
    ($T:ident, $default_epsilon:expr) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn default_epsilon() -> $T {
                $default_epsilon
            }

            #[inline]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                (if self > other {
                    self - other
                } else {
                    other - self
                }) <= epsilon
            }
        }
    };
}

impl_unsigned_abs_diff_eq!(u8, 0);
impl_unsigned_abs_diff_eq!(u16, 0);
impl_unsigned_abs_diff_eq!(u32, 0);
impl_unsigned_abs_diff_eq!(u64, 0);
impl_unsigned_abs_diff_eq!(u128, 0);
impl_unsigned_abs_diff_eq!(usize, 0);

macro_rules! impl_signed_abs_diff_eq {
    ($T:ident, $default_epsilon:expr) => {
        impl AbsDiffEq for $T {
            type Epsilon = $T;

            #[inline]
            fn default_epsilon() -> $T {
                $default_epsilon
            }

            #[inline]
            #[allow(unused_imports)]
            fn abs_diff_eq(&self, other: &$T, epsilon: $T) -> bool {
                use num_traits::float::FloatCore;
                $T::abs(self - other) <= epsilon
            }
        }
    };
}

impl_signed_abs_diff_eq!(i8, 0);
impl_signed_abs_diff_eq!(i16, 0);
impl_signed_abs_diff_eq!(i32, 0);
impl_signed_abs_diff_eq!(i64, 0);
impl_signed_abs_diff_eq!(i128, 0);
impl_signed_abs_diff_eq!(isize, 0);
impl_signed_abs_diff_eq!(f32, f32::EPSILON);
impl_signed_abs_diff_eq!(f64, f64::EPSILON);
