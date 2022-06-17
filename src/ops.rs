//! Implementations of basic arithmetic operations on `Size<T>`. Only operations that make sense are
//! implemented, e.g. while it is OK to add two `Size` objects, it does not make sense to multiply
//! them. Meanwhile, `17MiB / 2` is perfectly rational and returns a size equivalent  to `3.5 MiB`,
//! but `12KB + 14` isn't (as the addition of a scalar value to a sized type is undefined) -- on the
//! other hand, `12KB + 14B` is both supported and perfectly fine.
//!
//! Some examples of supported mathematical operations:
//!
//! ```
//! use size::Size;
//!
//! // Perform scalar multiplication/division on a `Size`
//! let s1 = Size::MiB(13) / 2;
//! assert_eq!(s1, Size::MiB(6.5_f32));
//!
//! // Perform addition or subtraction of two `Size` instances,
//! // regardless of their underlying types
//! let s2 = Size::KiB(4) + Size::MB(8);
//! assert_eq!(s2.bytes(), 8004096);
//!
//! // Express the negative difference between two sizes
//! let s3 = Size::MiB(12) - Size::MiB(14.2_f64);
//! assert_eq!(s3, Size::KiB(-2252.8));
//! ```
//!
//! Some other things you cannot do are multiply/divide two sizes (did you mean to multiply one size
//! by a scalar value instead?), add/subtract scalar values from sizes (you can call `size.bytes()`
//! then do all the scalar math you like, however), or perform mathematical operations that exceed
//! the bounds of the intermediate type (`f64` by default or `i64` if `no_std` mode is used).
//!
//! A current limitation of this crate that may be revisited at a later date is that mathematical
//! operations (or textual representation, for that matter) of that result in a size that exceeds
//! the bounds of an `i64` are not supported (i.e. they will not be promoted to a
//! floating-point-backed `Size` instance) and will panic in debug mode or silently fail with
//! undefined results in release mode. This applies even if the `Size` being interacted with is
//! currently backed by a double-precision type (e.g. using `Size<f64>`).

use crate::{Size, Underlying};
use core::ops::{Add, Div, Mul, Sub};
use num_traits::AsPrimitive;

impl<T, U> Add<&Size<U>> for &Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn add(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<Size<U>> for &Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn add(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<&Size<U>> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn add(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Add<Size<U>> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn add(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() + other.bytes())
    }
}

impl<T, U> Sub<&Size<U>> for &Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn sub(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T, U> Sub<Size<U>> for &Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn sub(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T, U> Sub<&Size<U>> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn sub(self, other: &Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T, U> Sub<Size<U>> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn sub(self, other: Size<U>) -> Self::Output {
        Size::Bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T, U> Mul<U> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn mul(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as Underlying * other.as_()) as i64)
    }
}

impl<T, U> Mul<U> for &Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn mul(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as Underlying * other.as_()) as i64)
    }
}

/// Defined to allow multiplying an untyped number by a Size<T>, because
/// multiplication should be commutative.
impl<T> Mul<Size<T>> for i64
where
    T: AsPrimitive<Underlying>,
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
    T: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn mul(self, other: &Size<T>) -> Self::Output {
        Size::Bytes((self as i64 * other.bytes()) as i64)
    }
}

/// Defined to allow multiplying an untyped number by a Size<T>, because
/// multiplication should be commutative.
#[cfg(feature = "std")]
impl<T> Mul<Size<T>> for f64
where
    T: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn mul(self, other: Size<T>) -> Self::Output {
        Size::Bytes((self * other.bytes() as f64) as i64)
    }
}

/// Defined to allow multiplying an untyped number by a Size<T>, because
/// multiplication should be commutative.
#[cfg(feature = "std")]
impl<T> Mul<&Size<T>> for f64
where
    T: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn mul(self, other: &Size<T>) -> Self::Output {
        Size::Bytes((self * other.bytes() as f64) as i64)
    }
}

impl<T, U> Div<U> for &Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn div(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as Underlying / other.as_()) as i64)
    }
}

impl<T, U> Div<U> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    type Output = Size<i64>;

    fn div(self, other: U) -> Self::Output {
        Size::Bytes((self.bytes() as Underlying / other.as_()) as i64)
    }
}
