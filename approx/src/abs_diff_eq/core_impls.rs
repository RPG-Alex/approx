use core::cell;

use super::AbsDiffEq;

impl<T: AbsDiffEq> AbsDiffEq for Option<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Option<T>, epsilon: T::Epsilon) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => T::abs_diff_eq(a, b, epsilon),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: AbsDiffEq, E: AbsDiffEq> AbsDiffEq for Result<T, E> {
    type Epsilon = (T::Epsilon, E::Epsilon);

    #[inline]
    fn default_epsilon() -> (T::Epsilon, E::Epsilon) {
        (T::default_epsilon(), E::default_epsilon())
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Result<T, E>, epsilon: (T::Epsilon, E::Epsilon)) -> bool {
        match (self, other) {
            (Ok(a), Ok(b)) => T::abs_diff_eq(a, b, epsilon.0),
            (Err(a), Err(b)) => E::abs_diff_eq(a, b, epsilon.1),
            _ => false,
        }
    }
}

impl<'a, T: AbsDiffEq + ?Sized> AbsDiffEq for &'a T {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&'a T, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(*self, *other, epsilon)
    }
}

impl<'a, T: AbsDiffEq + ?Sized> AbsDiffEq for &'a mut T {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &&'a mut T, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(*self, *other, epsilon)
    }
}

impl<T: AbsDiffEq + Copy> AbsDiffEq for cell::Cell<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &cell::Cell<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.get(), &other.get(), epsilon)
    }
}

impl<T: AbsDiffEq + ?Sized> AbsDiffEq for cell::RefCell<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &cell::RefCell<T>, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.borrow(), &other.borrow(), epsilon)
    }
}
