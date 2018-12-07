//! Implementations of basic arithmetic operations on `Size<T>`.
//! Only operations that make sense are implemented, e.g. while it is OK to add
//! two `Size` objects, it does not make sense to multiply them. Meanwhile,
//! `17MiB / 2` is perfectly rational, but `12KB + 14` isn't (the RHS unit isn't
//! defined).

use crate::Size;
use core::ops::{Add, Div, Mul};
use num_traits::ToPrimitive;

impl<T, U> Add<&Size<U>> for &Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    type Output = Size<u64>;

    fn add(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<Size<U>> for &Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    type Output = Size<u64>;

    fn add(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<&Size<U>> for Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    type Output = Size<u64>;

    fn add(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<Size<U>> for Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    type Output = Size<u64>;

    fn add(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Mul<U> for Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    type Output = Size<u64>;

    fn mul(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as f64 * other.to_f64().unwrap()) as u64)
    }
}

impl<T, U> Mul<U> for &Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    type Output = Size<u64>;

    fn mul(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as f64 * other.to_f64().unwrap()) as u64)
    }
}

/// Defined to allow multiplying an untyped number by a Size<T>, because
/// multiplication should be commutative.
impl<T> Mul<Size<T>> for i64
where
    T: ToPrimitive,
{
    type Output = Size<u64>;

    fn mul(self, other: Size<T>) -> Self::Output {
        Size::Bytes((self as u64 * other.bytes()) as u64)
    }
}

impl<T> Mul<Size<T>> for f64
where
    T: ToPrimitive,
{
    type Output = Size<u64>;

    fn mul(self, other: Size<T>) -> Self::Output {
        Size::Bytes((self * other.bytes() as f64) as u64)
    }
}

impl<T, U> Div<U> for &Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    type Output = Size<u64>;

    fn div(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as f64 / other.to_f64().unwrap()) as u64)
    }
}

impl<T, U> Div<U> for Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    type Output = Size<u64>;

    fn div(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as f64 / other.to_f64().unwrap()) as u64)
    }
}
