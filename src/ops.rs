//! Implementations of basic arithmetic operations on `Size<T>`. Only operations that make sense are
//! implemented, e.g. while it is OK to add two `Size` objects, it does not make sense to multiply
//! them. Meanwhile, `17MiB / 2` is perfectly rational, but `12KB + 14` isn't (the RHS unit isn't
//! defined).

use crate::Size;
use core::ops::{Add, Div, Mul, Sub};
use num_traits::AsPrimitive;

trait PrimFloat {
}

impl PrimFloat for f32 {}
impl PrimFloat for f64 {}

impl<T, U> Add<&Size<U>> for &Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn add(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<Size<U>> for &Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn add(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<&Size<U>> for Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn add(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<Size<U>> for Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn add(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Sub<&Size<U>> for &Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn sub(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T, U> Sub<Size<U>> for &Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn sub(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T, U> Sub<&Size<U>> for Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn sub(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T, U> Sub<Size<U>> for Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn sub(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T, U> Mul<U> for Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn mul(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as f64 * other.as_()) as i64)
    }
}

impl<T, U> Mul<U> for &Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn mul(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as f64 * other.as_()) as i64)
    }
}

/// Defined to allow multiplying an untyped number by a Size<T>, because
/// multiplication should be commutative.
impl<T> Mul<Size<T>> for i64
where
    T: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn mul(self, other: Size<T>) -> Self::Output {
        Size::Bytes((self as i64 * other.bytes()) as i64)
    }
}

/// Defined to allow multiplying an untyped number by a Size<T>, because
/// multiplication should be commutative.
impl<T> Mul<&Size<T>> for i64
where
    T: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn mul(self, other: &Size<T>) -> Self::Output {
        Size::Bytes((self as i64 * other.bytes()) as i64)
    }
}

/// Defined to allow multiplying an untyped number by a Size<T>, because
/// multiplication should be commutative.
impl<T> Mul<Size<T>> for f64
where
    T: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn mul(self, other: Size<T>) -> Self::Output {
        Size::Bytes((self * other.bytes() as f64) as i64)
    }
}

/// Defined to allow multiplying an untyped number by a Size<T>, because
/// multiplication should be commutative.
impl<T> Mul<&Size<T>> for f64
where
    T: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn mul(self, other: &Size<T>) -> Self::Output {
        Size::Bytes((self * other.bytes() as f64) as i64)
    }
}

impl<T, U> Div<U> for &Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn div(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as f64 / other.as_()) as i64)
    }
}

impl<T, U> Div<U> for Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    type Output = Size<i64>;

    fn div(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as f64 / other.as_()) as i64)
    }
}
