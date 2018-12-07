//! Implementations of basic arithmetic operations on `Size<T>`.

use crate::Size;
use core::ops::Add;
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
