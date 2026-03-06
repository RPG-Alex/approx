use super::AbsDiffEq;
#[cfg(feature = "vec_impl")]
use alloc::vec::Vec;
#[cfg(feature = "indexmap_impl")]
use core::hash::{BuildHasher, Hash};
#[cfg(feature = "indexmap_impl")]
use indexmap::IndexMap;

impl<A, B> AbsDiffEq<[B]> for [A]
where
    A: AbsDiffEq<B>,
    A::Epsilon: Clone,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn default_epsilon() -> A::Epsilon {
        A::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &[B], epsilon: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other).all(|(x, y)| A::abs_diff_eq(x, y, epsilon.clone()))
    }
}

#[cfg(feature = "array_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "array_impl")))]
impl<A, B, const N: usize> AbsDiffEq<[B; N]> for [A; N]
where
    A: AbsDiffEq<B>,
    A::Epsilon: Clone,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn default_epsilon() -> A::Epsilon {
        A::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &[B; N], epsilon: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other).all(|(x, y)| A::abs_diff_eq(x, y, epsilon.clone()))
    }
}

#[cfg(feature = "vec_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "vec_impl")))]
impl<A, B> AbsDiffEq<Vec<B>> for Vec<A>
where
    A: AbsDiffEq<B>,
    A::Epsilon: Clone,
{
    type Epsilon = A::Epsilon;

    #[inline]
    fn default_epsilon() -> A::Epsilon {
        A::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Vec<B>, epsilon: A::Epsilon) -> bool {
        self.len() == other.len()
            && Iterator::zip(self.iter(), other).all(|(x, y)| A::abs_diff_eq(x, y, epsilon.clone()))
    }
}

#[cfg(feature = "indexmap_impl")]
#[cfg_attr(docsrs, doc(cfg(feature = "indexmap_impl")))]
impl<K, V1, V2, S1, S2> AbsDiffEq<IndexMap<K, V2, S2>> for IndexMap<K, V1, S1>
where
    K: Hash + Eq,
    V1: AbsDiffEq<V2>,
    V1::Epsilon: Clone,
    S1: BuildHasher,
    S2: BuildHasher,
{
    type Epsilon = V1::Epsilon;

    #[inline]
    fn default_epsilon() -> V1::Epsilon {
        V1::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &IndexMap<K, V2, S2>, epsilon: Self::Epsilon) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, value)| {
                other
                    .get(key)
                    .map_or(false, |v| V1::abs_diff_eq(value, v, epsilon.clone()))
            })
    }
}
