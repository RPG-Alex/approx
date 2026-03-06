#[cfg(any(feature = "num_complex", feature = "ordered_float"))]
use super::AbsDiffEq;

#[cfg(feature = "num_complex")]
use num_complex::Complex;
#[cfg(feature = "ordered_float")]
use num_traits::Float;
#[cfg(feature = "ordered_float")]
use ordered_float::{NotNan, OrderedFloat};

#[cfg(feature = "num_complex")]
#[cfg_attr(docsrs, doc(cfg(feature = "num_complex")))]
impl<T: AbsDiffEq> AbsDiffEq for Complex<T>
where
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Complex<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.re, &other.re, epsilon.clone())
            && T::abs_diff_eq(&self.im, &other.im, epsilon)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: AbsDiffEq + Copy> AbsDiffEq for NotNan<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.into_inner(), &other.into_inner(), epsilon)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: AbsDiffEq + Float + ordered_float::FloatCore> AbsDiffEq<T> for NotNan<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &T, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.into_inner(), other, epsilon)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: AbsDiffEq + Float + ordered_float::FloatCore> AbsDiffEq for OrderedFloat<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.into_inner(), &other.into_inner(), epsilon)
    }
}

#[cfg(feature = "ordered_float")]
#[cfg_attr(docsrs, doc(cfg(feature = "ordered_float")))]
impl<T: AbsDiffEq + Float + ordered_float::FloatCore> AbsDiffEq<T> for OrderedFloat<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &T, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.into_inner(), other, epsilon)
    }
}
