//! Implementations of basic arithmetic operations on `Size`. Only operations that make sense are
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
//! let s1 = Size::from_mib(13) / 2;
//! assert_eq!(s1, Size::from_mib(6.5_f32));
//!
//! // Perform addition or subtraction of two `Size` instances,
//! // regardless of their underlying types
//! let s2 = Size::from_kib(4) + Size::from_mb(8);
//! assert_eq!(s2.bytes(), 8004096);
//!
//! // Express the negative difference between two sizes
//! let s3 = Size::from_mib(12) - Size::from_mib(14.2_f64);
//! assert_eq!(s3, Size::from_kib(-2252.8));
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
//! undefined results in release mode.

use crate::{AsIntermediate, Intermediate, Size};
use core::ops::{Add, Div, Mul, Sub};

impl Add<&Size> for &Size
{
    type Output = Size;

    fn add(self, other: &Size) -> Self::Output {
        Size::from_bytes(self.bytes() + other.bytes())
    }
}

impl Add<Size> for &Size
{
    type Output = Size;

    fn add(self, other: Size) -> Self::Output {
        Size::from_bytes(self.bytes() + other.bytes())
    }
}

impl Add<&Size> for Size
{
    type Output = Size;

    fn add(self, other: &Size) -> Self::Output {
        Size::from_bytes(self.bytes() + other.bytes())
    }
}

impl Add<Size> for Size
{
    type Output = Size;

    fn add(self, other: Size) -> Self::Output {
        Size::from_bytes(self.bytes() + other.bytes())
    }
}

impl Sub<&Size> for &Size
{
    type Output = Size;

    fn sub(self, other: &Size) -> Self::Output {
        Size::from_bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl Sub<Size> for &Size
{
    type Output = Size;

    fn sub(self, other: Size) -> Self::Output {
        Size::from_bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl Sub<&Size> for Size
{
    type Output = Size;

    fn sub(self, other: &Size) -> Self::Output {
        Size::from_bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl Sub<Size> for Size
{
    type Output = Size;

    fn sub(self, other: Size) -> Self::Output {
        Size::from_bytes(self.bytes() as i64 - other.bytes() as i64)
    }
}

impl<T> Mul<T> for Size
where
    T: AsIntermediate,
{
    type Output = Size;

    fn mul(self, other: T) -> Self::Output {
        Size::from_bytes((self.bytes() as Intermediate * other.as_()) as i64)
    }
}

impl<T> Mul<T> for &Size
where
    T: AsIntermediate,
{
    type Output = Size;

    fn mul(self, other: T) -> Self::Output {
        Size::from_bytes((self.bytes() as Intermediate * other.as_()) as i64)
    }
}

/// Defined to allow multiplying an untyped integral literal by a Size, because
/// multiplication should be commutative.
impl Mul<Size> for i64
{
    type Output = Size;

    fn mul(self, other: Size) -> Self::Output {
        Size::from_bytes((self as Intermediate * other.bytes() as Intermediate) as i64)
    }
}

/// Defined to allow multiplying an untyped integral literal by a Size, because
/// multiplication should be commutative.
impl Mul<&Size> for i64
{
    type Output = Size;

    fn mul(self, other: &Size) -> Self::Output {
        Size::from_bytes((self as Intermediate * other.bytes() as Intermediate) as i64)
    }
}

/// Defined to allow multiplying an untyped floating scalar by a Size, because
/// multiplication should be commutative.
impl Mul<Size> for f64
{
    type Output = Size;

    fn mul(self, other: Size) -> Self::Output {
        Size::from_bytes((self as Intermediate * other.bytes() as Intermediate) as i64)
    }
}

/// Defined to allow multiplying an untyped floating scalar by a Size, because
/// multiplication should be commutative.
impl Mul<&Size> for f64
{
    type Output = Size;

    fn mul(self, other: &Size) -> Self::Output {
        Size::from_bytes((self as Intermediate * other.bytes() as Intermediate) as i64)
    }
}

impl<T> Div<T> for &Size
where
    T: AsIntermediate,
{
    type Output = Size;

    fn div(self, other: T) -> Self::Output {
        Size::from_bytes((self.bytes() as Intermediate / other.as_()) as i64)
    }
}

impl<T> Div<T> for Size
where
    T: AsIntermediate,
{
    type Output = Size;

    fn div(self, other: T) -> Self::Output {
        Size::from_bytes((self.bytes() as Intermediate / other.as_()) as i64)
    }
}
