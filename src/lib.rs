#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! This crate provides an ergonomic, type-safe, and aesthetically-pleasing [`Size`] type that can
//! be used to express, format, or operate on sizes. While it was initially created to make it
//! painless to "pretty print" file sizes (by automatically determining which unit and with what
//! precision a file size should be textually "written out" or formatted), it has expanded in scope
//! to make it easier and safer to perform the different types of operations that would arise when
//! dealing with sizes.
//!
//! For almost all users, the only surface of interaction with this crate will take place via the
//! `Size` type, which can be used to create a strongly-typed representation of a file size (or any
//! other "size" you need to deal with in the abstract). This crate's API is intended to be as
//! natural and intuitive as possible, providing sensible defaults with zero boilerplate but also
//! allowing the developer to manually control aspects how sizes are expressed as text if needed.
//!
//! The core [`Size`] type is a simple wrapper around a signed numeric value - it can be initialized
//! using whatever primitive numeric type you wish, e.g. constructing a `Size` from an `i64` or from
//! a `foo: f64` number of kilobytes.
//!
//! ## Using this crate and creating a `Size` object
//!
//! To use this crate, you only need to place `use size::Size` at the top of your rust code, then
//! create a `Size` from a constructor/initializer that matches the size you have on hand. Both
//! base-2 (KiB, MiB, etc) and base-10 (KB, MB, etc) units are supported and are exposed via the
//! same API. You can either use the abbreviated form of the unit to instantiate your type, or use
//! the full unit name to be more expressive. Here's an example:
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use size::Size;
//!
//! // Create a strongly-typed size object. We don't even need to specify a numeric type!
//! let file1_size = Size::from_bytes(200);
//! // Create another Size instance, this time from a floating-point literal:
//! let file2_size = Size::from_kb(20.1);
//! ```
//!
//! You can obtain a scalar `i64` value equal to the total number of bytes described by a
//! `Size` instance by calling [`Size::bytes()`] (see link for more info):
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use size::Size;
//!
//! let file_size = Size::from_gibibytes(4);
//! assert_eq!(file_size.bytes(), 4_294_967_296);
//! ```
//!
//! All `Size` types can be directly compared (both for order and equality) to one another (or to
//! references of one another), regardless of their original type:
//! ```
//! use size::Size;
//!
//! let size1 = Size::from_kib(4 as u8);
//! let size2 = Size::from_bytes(4096 as i64);
//! assert_eq!(size1, size2);
//!
//! let size1 = Size::from_kib(7);
//! let size2 = Size::from_kb(7);
//! assert!(&size2 < &size1);
//! ```
//!
//! ## Textual representation
//!
//! The majority of users will be interested in this crate for its ability to "pretty print" sizes
//! with little ceremony and great results. All `Size` instances implement both
//! [`std::fmt::Display`] and [`std::fmt::Debug`], so you can just directly `format!(...)` or
//! `println!(...)` with whatever `Size` you have on hand:
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use size::Size;
//!
//! let file_size = Size::from_bytes(1_340_249);
//! let textual = format!("{}", file_size); // "1.28 MiB"
//! assert_eq!(textual.as_str(), "1.28 MiB");
//! ```
//!
//! [`Size::to_string()`](ToString::to_string) can be used to directly return a `String` containing
//! the formatted, human-readable size, instead of needing to use the `format!()` macro or similar:
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use size::Size;
//!
//! let file_size = Size::from_bytes(1_340_249);
//! assert_eq!(file_size.to_string(), "1.28 MiB".to_string());
//! ```
//!
//! For fine-grained control over how a size is formatted and displayed, you can manually use the
//! [`Size::format()`] function, which returns a [`FormattableSize`](crate::fmt::FormattableSize)
//! implementing the builder model to allow you to change one or more properties of how a `Size`
//! is formatted:
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use size::{Size, Base, Style};
//!
//! let file_size = Size::from_bytes(1_340_249); // same as before
//! let textual_size = file_size.format()
//!     .with_base(Base::Base10)
//!     .with_style(Style::FullLowercase)
//!     .to_string();
//! assert_eq!(textual_size, "1.34 megabytes".to_string());
//! ```
//!
//! It is also possible to create and configure a standalone [`SizeFormatter`] that can be reused to
//! format many sizes in a single, consistent style. This should not be seen as an alternative to
//! wrapping file sizes in strongly-typed `Size` structs, which should always be the initial
//! instinct.
//!
//! ## Mathematical operations
//!
//! You can perform mathematical operations on `Size` types and the type safety makes sure that
//! what you're doing makes sense:
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
#![cfg_attr(feature = "std", doc = "```")]
//! use size::Size;
//!
//! let sum = Size::from_mib(2) + Size::from_kib(200);
//! assert_eq!(sum, Size::from_mb(2.301_952));
//!
//! let size = Size::from_gb(4.2) / 2;
//! assert_eq!(size, Size::from_gb(2.1));
//! ```
//!
//! See the documentation of the [`ops`] module for more on this topic.
//!
//! ## Parsing sizes from text
//!
//! The [`Size::from_str()`] function can be used to convert the most commonly encountered textual
//! representations of file sizes into properly typed `Size` objects, with flexible support for
//! various input whitespace formatting, abbreviated/full unit names, mixed upper/lower-case
//! representation, etc.
//!
//! ## Crate features
//!
//! This crate currently has one feature (`std`), enabled by default. If compiled with
//! `--no-default-features` or used as a dependency with default features disabled, the crate
//! becomes `no_std` compatible. When used in `no_std` mode, the following restrictions and
//! limitations are observed:
//!
//! * All formatting/stringification of `Size` types is disabled.
//! * `Size` no longer implements [`std::fmt::Display`] (`core::fmt::Debug` is still implemented).
//! * The intermediate type used for mathematical operations on `Size` types is changed from `f64`
//! to `i64` so that no implicit floating-point math is performed. To prevent inadvertent loss of
//! precision, it is forbidden to pass in floating point values to the `Size` API under `no_std`
//! mode.
//!
//! ## Base-2 and Base-10 constants
//!
//! You can individually use constants like `size::KiB` or `size::GB` directly or import all
//! constants into scope with `use size::consts::*` (or just `use size::*`, but that also imports
//! the types and traits defined by this crate, too).
//!
//! ## Serialization support
//!
//! If the crate is compiled with the optional (default: disabled) `serde` feature, the `Size` type
//! may be serialized/deserialized directly to/from payloads via the `serde` crate. The `Size` type
//! is treated as a transparent new-type around `u64` for serialization purposes (i.e. it serializes
//! directly to the number of bytes, not as a struct with the number of bytes as a member/field);
//! this allows deserializing payloads from various APIs or other languages that typically do not
//! use strongly-typed `Size` objects to denote (file) size.
//!
//! As an example, `struct File { name: String, size: Size } ` will serialize to `{ name: "name",
//! size: 1234 }` instead of `{ name: "name", size: { bytes: 1234 }`.

#[cfg(feature = "std")]
pub mod fmt;
#[cfg(feature = "std")]
mod from_str;
pub mod ops;
#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_nostd;

pub use crate::consts::*;
#[cfg(feature = "std")]
pub use crate::fmt::{Base, SizeFormatter, Style};
#[cfg(feature = "std")]
pub use crate::from_str::ParseSizeError;
use crate::sealed::AsIntermediate;

#[cfg(feature = "std")]
type Intermediate = f64;
#[cfg(not(feature = "std"))]
type Intermediate = i64;

#[cfg(feature = "std")]
const DEFAULT_BASE: Base = Base::Base2;
#[cfg(feature = "std")]
const DEFAULT_STYLE: Style = Style::Default;

mod sealed {
    use super::Intermediate;

    pub trait AsIntermediate: Sized {
        // This is the same name and signature as `AsPrimitive` trait from the `num_traits` crate
        fn as_(self) -> Intermediate;
    }

    macro_rules! as_intermediate {
        ($type:ty) => {
            impl AsIntermediate for $type {
                fn as_(self) -> Intermediate {
                    use core::mem::size_of;
                    const SIGNED_MAX: $type = Intermediate::MAX as $type;

                    // A separate implementation is required for no_std's intermediate i64 to make
                    // sure u64::MAX is clamped to i64::MAX rather than cast directly to -1. The
                    // first three checks should be elided per impl via compile-time optimization.
                    if cfg!(not(feature = "std")) // we are in no_std mode
                        && <$type>::MIN == 0 as $type // it's an unsigned type
                        && size_of::<Intermediate>() >= size_of::<$type>() // with a greater +range
                        && self > SIGNED_MAX // and exceeds our max
                    {
                        Intermediate::MAX
                    } else {
                        self as Intermediate
                    }
                }
            }
        };
    }

    as_intermediate!(u8);
    as_intermediate!(u16);
    as_intermediate!(u32);
    as_intermediate!(u64);
    as_intermediate!(usize);
    as_intermediate!(i8);
    as_intermediate!(i16);
    as_intermediate!(i32);
    as_intermediate!(i64);
    as_intermediate!(isize);
    #[cfg(feature = "std")]
    as_intermediate!(f32);
    #[cfg(feature = "std")]
    as_intermediate!(f64);
}

/// A collection of constants for base-2 and base-10 units.
///
/// These can be used in a `const` context in conjunction with the `const` [`Size::from_const()`]
/// function to create strongly-sized `Size` objects expressing various sizes, e.g.
///
/// ```
/// use size::Size;
/// use size::consts::*;
///
/// pub const TOTAL_SIZE: Size = Size::from_const(3 * MiB);
/// ```
pub mod consts {
    #![allow(non_upper_case_globals)]

    /// Basic "byte" constant, used across all bases.
    pub const BYTE: i64 = 1;
    /// Base-10 "kilobyte" constant, equal to 1000 bytes.
    pub const KILOBYTE: i64 = 1000 * BYTE;
    /// Base-10 "megabyte" constant, equal to 1000 kilobytes.
    pub const MEGABYTE: i64 = 1000 * KILOBYTE;
    /// Base-10 "gigabyte" constant, equal to 1000 megabytes.
    pub const GIGABYTE: i64 = 1000 * MEGABYTE;
    /// Base-10 "terabyte" constant, equal to 1000 gigabytes.
    pub const TERABYTE: i64 = 1000 * GIGABYTE;
    /// Base-10 "petabyte" constant, equal to 1000 terabytes.
    pub const PETABYTE: i64 = 1000 * TERABYTE;
    /// Base-10 "exabyte" constant, equal to 1000 petabytes.
    pub const EXABYTE: i64 = 1000 * PETABYTE;

    /// Abbreviated "byte" constant. Identical to [`BYTE`].
    pub const B: i64 = BYTE;
    /// Abbreviated base-10 "kilobyte" constant, equal to 1000 bytes. Identical to [`KILOBYTE`].
    pub const KB: i64 = KILOBYTE;
    /// Abbreviated base-10 "megabyte" constant, equal to 1000 kilobytes. Identical to [`MEGABYTE`].
    pub const MB: i64 = MEGABYTE;
    /// Abbreviated base-10 "gigabyte" constant, equal to 1000 megabytes. Identical to [`GIGABYTE`].
    pub const GB: i64 = GIGABYTE;
    /// Abbreviated base-10 "terabyte" constant, equal to 1000 gigabytes. Identical to [`TERABYTE`].
    pub const TB: i64 = TERABYTE;
    /// Abbreviated base-10 "petabyte" constant, equal to 1000 terabytes. Identical to [`PETABYTE`].
    pub const PB: i64 = PETABYTE;
    /// Abbreviated base-10 "exabyte" constant, equal to 1000 petabytes. Identical to [`EXABYTE`].
    pub const EB: i64 = EXABYTE;

    /// Base-2 "kibibyte" constant, equal to 2^10 bytes.
    pub const KIBIBYTE: i64 = 1 << 10;
    /// Base-2 "mebibyte" constant, equal to 2^20 bytes.
    pub const MEBIBYTE: i64 = 1 << 20;
    /// Base-2 "gibibyte" constant, equal to 2^30 bytes.
    pub const GIBIBYTE: i64 = 1 << 30;
    /// Base-2 "tebibyte" constant, equal to 2^40 bytes.
    pub const TEBIBYTE: i64 = 1 << 40;
    /// Base-2 "pebibyte" constant, equal to 2^50 bytes.
    pub const PEBIBYTE: i64 = 1 << 50;
    /// Base-2 "exbibyte" constant, equal to 2^60 bytes.
    pub const EXBIBYTE: i64 = 1 << 60;

    /// Abbreviated base-2 "kibibyte" constant, equal to 1024 bytes. Identical to [`KIBIBYTE`].
    pub const KiB: i64 = KIBIBYTE;
    /// Abbreviated base-2 "mebibyte" constant, equal to 1024 kibibytes. Identical to [`MEBIBYTE`].
    pub const MiB: i64 = MEBIBYTE;
    /// Abbreviated base-2 "gibibyte" constant, equal to 1024 mebibytes. Identical to [`GIBIBYTE`].
    pub const GiB: i64 = GIBIBYTE;
    /// Abbreviated base-2 "tebibyte" constant, equal to 1024 gibibytes. Identical to [`TEBIBYTE`].
    pub const TiB: i64 = TEBIBYTE;
    /// Abbreviated base-2 "pebibyte" constant, equal to 1024 tebibytes. Identical to [`PEBIBYTE`].
    pub const PiB: i64 = PEBIBYTE;
    /// Abbreviated base-2 "exbibyte" constant, equal to 1024 pebibytes. Identical to [`EXBIBYTE`].
    pub const EiB: i64 = EXBIBYTE;
}

/// `Size` is the core type exposed by this crate and allows the developer to express a file size
/// (or the general concept of a "size") as a strongly-typed, convertible type that can be used for
/// textual formatting ("pretty printing") and mathematical operations.
///
/// A size can be created in terms of any supported unit and an associated numeric value of any
/// type.
#[cfg_attr(not(feature = "std"), doc = "```ignore")]
#[cfg_attr(feature = "std", doc = "```")]
/// use size::Size;
///
/// // Identical sizes expressed in different units with different primitive types:
/// assert_eq!(Size::from_kibibytes(2_u8), Size::from_kilobytes(2.048_f64));
/// ```
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Size {
    bytes: i64,
}

impl Size {
    /// Initialize a `Size` from the provided value, in bytes. This is a constant function and may
    /// be used in a `const` context.
    ///
    /// Unlike the other "from" functions (e.g. [`from_kilobytes()`](Size::from_kilobytes())), it is
    /// not generic because
    /// a) trait methods (required to use a generic type) may not be declared as `const`, and
    /// b) it's always safe to use `as i64` on whatever type you're actually passing into
    /// `from_bytes()` without any (additional) loss of precision as compared to passing in an
    /// arbitrary numeric type, since there is no math required to calculate the equivalent size in
    /// bytes.
    ///
    /// To further illustrate this point, let's look at this hypothetical initialization of a `Size`
    /// from a floating-point literal: `let s = Size::from_kib(2.5);` - when the conversion from
    /// "2.5 KiB" to "bytes" happens internally, the result is equivalent to `(2.5 * 1024.0) as i64`
    /// and yields the correct result of 2560 bytes. But if `from_kib` weren't generic and you
    /// needed to use `as i64` (i.e. `Size::from_kib(2.5 as i64)`), the calculated size in bytes
    /// would start from an already-truncated `2_i64` and yield an incorrect answer of 2048 bytes
    /// (`(2.5 as i64) * 1024`). However, with `from_bytes()`, there can be no loss of precision
    /// (or, pedantically, even truncation) when `as i64` is used since the file size, expressed in
    /// bytes, must always be a whole number; this means it is safe to perform the integer
    /// conversion/rounding at the call site itself and `Size::from_const(float_val as i64)` would
    /// necessarily always yield the same result as the generic/type-agnostic
    /// `Size::from_bytes::<f64>(float_val)`.
    pub const fn from_const(bytes: i64) -> Self {
        Self { bytes }
    }

    /// Initialize a `Size` from the provided value, in bytes.
    pub fn from_bytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: value.as_() as i64,
        }
    }

    /// Express a size in kilobytes. Actual size is 10^3 \* the value.
    pub fn from_kilobytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * KILOBYTE as Intermediate) as i64,
        }
    }

    /// Express a size in megabytes. Actual size is 10^6 \* the value.
    pub fn from_megabytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * MEGABYTE as Intermediate) as i64,
        }
    }

    /// Express a size in gigabytes. Actual size is 10^9 \* the value.
    pub fn from_gigabytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * GIGABYTE as Intermediate) as i64,
        }
    }

    /// Express a size in terabytes. Actual size is 10^12 \* the value.
    pub fn from_terabytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * TERABYTE as Intermediate) as i64,
        }
    }

    /// Express a size in petabytes. Actual size is 10^15 \* the value.
    pub fn from_petabytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * PETABYTE as Intermediate) as i64,
        }
    }

    /// Express a size in exabytes. Actual size is 10^18 \* the value.
    pub fn from_exabytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * EXABYTE as Intermediate) as i64,
        }
    }

    #[inline]
    /// Express a size in kilobytes, as a shortcut for using [`Size::from_kilobytes()`].
    pub fn from_kb<T: AsIntermediate>(value: T) -> Self {
        Self::from_kilobytes(value)
    }
    #[inline]
    /// Express a size in megabytes, as a shortcut for using [`Size::from_megabytes()`].
    pub fn from_mb<T: AsIntermediate>(value: T) -> Self {
        Self::from_megabytes(value)
    }
    #[inline]
    /// Express a size in gigabytes, as a shortcut for using [`Size::from_gigabytes()`].
    pub fn from_gb<T: AsIntermediate>(value: T) -> Self {
        Self::from_gigabytes(value)
    }
    #[inline]
    /// Express a size in terabytes, as a shortcut for using [`Size::from_terabytes()`].
    pub fn from_tb<T: AsIntermediate>(value: T) -> Self {
        Self::from_terabytes(value)
    }
    #[inline]
    /// Express a size in petabytes, as a shortcut for using [`Size::from_petabytes()`].
    pub fn from_pb<T: AsIntermediate>(value: T) -> Self {
        Self::from_petabytes(value)
    }
    #[inline]
    /// Express a size in exabytes, as a shortcut for using [`Size::from_exabytes()`].
    pub fn from_eb<T: AsIntermediate>(value: T) -> Self {
        Self::from_exabytes(value)
    }

    /// Express a size in kibibytes. Actual size is 2^10 \* the value.
    pub fn from_kibibytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * KIBIBYTE as Intermediate) as i64,
        }
    }

    /// Express a size in mebibytes. Actual size is 2^20 \* the value.
    pub fn from_mebibytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * MEBIBYTE as Intermediate) as i64,
        }
    }

    /// Express a size in gibibytes. Actual size is 2^30 \* the value.
    pub fn from_gibibytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * GIBIBYTE as Intermediate) as i64,
        }
    }

    /// Express a size in tebibytes. Actual size is 2^40 \* the value.
    pub fn from_tebibytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * TEBIBYTE as Intermediate) as i64,
        }
    }

    /// Express a size in pebibytes. Actual size is 2^50 \* the value.
    pub fn from_pebibytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * PEBIBYTE as Intermediate) as i64,
        }
    }

    /// Express a size in exbibytes. Actual size is 2^60 \* the value.
    pub fn from_exbibytes<T: AsIntermediate>(value: T) -> Self {
        Self {
            bytes: (value.as_() * EXBIBYTE as Intermediate) as i64,
        }
    }

    #[inline]
    /// Express a size in kibibytes, as a shortcut for using [`Size::from_kibibytes()`].
    pub fn from_kib<T: AsIntermediate>(value: T) -> Self {
        Self::from_kibibytes(value)
    }
    #[inline]
    /// Express a size in mebibytes, as a shortcut for using [`Size::from_mebibytes()`].
    pub fn from_mib<T: AsIntermediate>(value: T) -> Self {
        Self::from_mebibytes(value)
    }
    #[inline]
    /// Express a size in gibibytes, as a shortcut for using [`Size::from_gibibytes()`].
    pub fn from_gib<T: AsIntermediate>(value: T) -> Self {
        Self::from_gibibytes(value)
    }
    #[inline]
    /// Express a size in tebibytes, as a shortcut for using [`Size::from_tebibytes()`].
    pub fn from_tib<T: AsIntermediate>(value: T) -> Self {
        Self::from_tebibytes(value)
    }
    #[inline]
    /// Express a size in pebibytes, as a shortcut for using [`Size::from_pebibytes()`].
    pub fn from_pib<T: AsIntermediate>(value: T) -> Self {
        Self::from_pebibytes(value)
    }
    #[inline]
    /// Express a size in exbibytes, as a shortcut for using [`Size::from_exbibytes()`].
    pub fn from_eib<T: AsIntermediate>(value: T) -> Self {
        Self::from_exbibytes(value)
    }
}

impl Size {
    #[inline]
    /// Returns the effective size in bytes of the type, useful for obtaining a plain/scalar
    /// representation of the full size represented by a [`Size`] object. This always returns an
    /// `i64` regardless of the underlying type originally used, to avoid (or at least mitigate)
    /// issues with integer overflow (e.g. when trying to retrieve `Size::from_tb(16_i32).bytes()`).
    ///
    /// Example:
    /// ```
    /// use size::Size;
    /// assert_eq!(Size::from_mib(4_u8).bytes(), 4_194_304 as i64);
    /// ```
    pub const fn bytes(&self) -> i64 {
        self.bytes
    }
}

// The original `size` approach was a rust enum with each unit expressed as a different variant, but
// that was never really a "rusty" solution and didn't actually match how size calculation was
// handled (with each value being converted to an f64/i64 before calculating the total bytes or the
// mathematical sum/difference/product/etc). The impl block below is for backwards
// source-compatibility purposes (with functions masquerading as enum variants).
#[doc(hidden)]
impl Size {
    #![allow(non_snake_case)]

    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_bytes() instead")]
    /// Express a size in bytes.
    pub fn Bytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_bytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_kibibytes() instead")]
    /// Express a size in kibibytes. Actual size is 2^10 \* the value.
    pub fn Kibibytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_kibibytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_kilobytes() instead")]
    /// Express a size in kilobytes. Actual size is 10^3 \* the value.
    pub fn Kilobytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_kilobytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_mebibytes() instead")]
    /// Express a size in mebibytes. Actual size is 2^20 \* the value.
    pub fn Mebibytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_mebibytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_megabytes() instead")]
    /// Express a size in megabytes. Actual size is 10^6 \* the value.
    pub fn Megabytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_megabytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_gibibytes() instead")]
    /// Express a size in gibibytes. Actual size is 2^30 \* the value.
    pub fn Gibibytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_gibibytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_gigabytes() instead")]
    /// Express a size in gigabytes. Actual size is 10^9 \* the value.
    pub fn Gigabytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_gigabytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_tebibytes() instead")]
    /// Express a size in tebibytes. Actual size is 2^40 \* the value.
    pub fn Tebibytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_tebibytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_terabytes() instead")]
    /// Express a size in terabytes. Actual size is 10^12 \* the value.
    pub fn Terabytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_terabytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_pebibytes() instead")]
    /// Express a size in pebibytes. Actual size is 2^50 \* the value.
    pub fn Pebibytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_pebibytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_petabytes() instead")]
    /// Express a size in petabytes. Actual size is 10^15 \* the value.
    pub fn Petabytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_petabytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_exbibytes() instead")]
    /// Express a size in exbibytes. Actual size is 2^60 \* the value.
    pub fn Exbibytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_exbibytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_exabytes() instead")]
    /// Express a size in exabytes. Actual size is 10^18 \* the value.
    pub fn Exabytes<T: AsIntermediate>(t: T) -> Self {
        Self::from_exabytes(t)
    }

    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_bytes() instead")]
    /// Express a size in bytes, as a shortcut for using [`Size::Bytes`].
    pub fn B<T: AsIntermediate>(t: T) -> Self {
        Self::from_bytes(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_kib() instead")]
    /// Express a size in kibibytes, as a shortcut for using [`Size::Kibibytes`].
    pub fn KiB<T: AsIntermediate>(t: T) -> Self {
        Self::from_kib(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_kb() instead")]
    /// Express a size in kilobytes, as a shortcut for using [`Size::Kilobytes`].
    pub fn KB<T: AsIntermediate>(t: T) -> Self {
        Self::from_kb(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_mib() instead")]
    /// Express a size in mebibytes, as a shortcut for using [`Size::Mebibytes`].
    pub fn MiB<T: AsIntermediate>(t: T) -> Self {
        Self::from_mib(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_mb() instead")]
    /// Express a size in megabytes, as a shortcut for using [`Size::Megabytes`].
    pub fn MB<T: AsIntermediate>(t: T) -> Self {
        Self::from_mb(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_gib() instead")]
    /// Express a size in gibibytes, as a shortcut for using [`Size::Gibibytes`].
    pub fn GiB<T: AsIntermediate>(t: T) -> Self {
        Self::from_gib(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_gb() instead")]
    /// Express a size in gigabytes, as a shortcut for using [`Size::Gigabytes`].
    pub fn GB<T: AsIntermediate>(t: T) -> Self {
        Self::from_gb(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_tib() instead")]
    /// Express a size in tebibytes, as a shortcut for using [`Size::Tebibytes`].
    pub fn TiB<T: AsIntermediate>(t: T) -> Self {
        Self::from_tib(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_tb() instead")]
    /// Express a size in terabytes, as a shortcut for using [`Size::Terabytes`].
    pub fn TB<T: AsIntermediate>(t: T) -> Self {
        Self::from_tb(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_pib() instead")]
    /// Express a size in pebibytes, as a shortcut for using [`Size::Pebibytes`].
    pub fn PiB<T: AsIntermediate>(t: T) -> Self {
        Self::from_pib(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_pb() instead")]
    /// Express a size in petabytes, as a shortcut for using [`Size::Petabytes`].
    pub fn PB<T: AsIntermediate>(t: T) -> Self {
        Self::from_pb(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_eib() instead")]
    /// Express a size in exbibytes, as a shortcut for using [`Size::Exbibytes`].
    pub fn EiB<T: AsIntermediate>(t: T) -> Self {
        Self::from_eib(t)
    }
    #[inline]
    #[deprecated(since = "0.3.0", note = "Use Size::from_eb() instead")]
    /// Express a size in exabytes, as a shortcut for using [`Size::Exabytes`].
    pub fn EB<T: AsIntermediate>(t: T) -> Self {
        Self::from_eb(t)
    }
}

impl core::fmt::Debug for Size {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "{} bytes", self.bytes())
    }
}
