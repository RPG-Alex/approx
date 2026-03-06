/// Equality that is defined using the absolute difference of two numbers.
///
/// For two numbers `a` and `b`, if `|a - b| < epsilon`, then the two numbers are considered
/// to be equal under the absolute difference equality.
///
/// `abs_diff_eq`, `abs_diff_ne`, `assert_abs_diff_eq`, and `assert_abs_diff_ne` macros
/// are all wrappers of the `abs_diff_eq` function in this trait.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate approx;
/// # fn main() {
/// assert_abs_diff_eq!(1.0f32, 1.00000001f32, epsilon = 1e-8);
/// assert_abs_diff_ne!(1.0f32, 1.0000001f32, epsilon = 1e-8);
/// # }
/// ```
pub trait AbsDiffEq<Rhs = Self>: PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    /// Used for specifying relative comparisons.
    type Epsilon;

    /// The default tolerance to use when testing values that are close together.
    ///
    /// This is used when no `epsilon` value is supplied to the
    /// [`abs_diff_eq!`](crate::abs_diff_eq), [`relative_eq!`](crate::relative_eq),
    /// or [`ulps_eq!`](crate::ulps_eq) macros.
    fn default_epsilon() -> Self::Epsilon;

    /// A test for equality that uses the absolute difference to compute the approximate
    /// equality of two numbers.
    fn abs_diff_eq(&self, other: &Rhs, epsilon: Self::Epsilon) -> bool;

    /// The inverse of [`AbsDiffEq::abs_diff_eq`].
    fn abs_diff_ne(&self, other: &Rhs, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}
