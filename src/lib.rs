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
//! The core [`Size`] type is a simple wrapper around any numeric type - you can use it with
//! whatever primitive numeric type you wish, e.g. constructing a `Size<i64>` or a `Size<f32>` from
//! whatever value you happen to have on hand. (Unlike many other generic-heavy rust crates, the
//! choice of underlying type won't cause any type mismatches down the line, and you can easily and
//! safely interoperate on a mixture of differently-backed `Size` types without fear.)
//!
//! ## Using this crate and creating a `Size` object
//! To use this crate, you only need to place a `using size::Size` at the top of your rust code,
//! then pick the unit that matches the size you have on hand. Both base-2 (KiB, MiB, etc) and
//! base-10 (KB, MB, etc) units are supported and are exposed via the same API. You can be either
//! use the abbreviated form of the unit to instantiate your type, or use the full unit name to be
//! more expressive. Here's an example:
//!
//! ```
//! use size::Size;
//!
//! // Create a strongly-typed size object. We don't even need to pick an underlying type!
//! let file1_size = Size::KiB(200);
//! // Create another Size instance, this time from a floating-point literal:
//! let file2_size = Size::Kilobytes(20.1);
//! ```
//!
//! You can obtain a scalar `i64` value equivalent to the total number of bytes described by a
//! `Size` instance by calling [`Size::bytes()`] (see link for more info):
//!
//! ```
//! use size::Size;
//!
//! let file_size = Size::GiB(4);
//! assert_eq!(file_size.bytes(), 4_294_967_296);
//! ```
//!
//! All `Size` types can be directly compared (both for ordering and for equality) to one another
//! (or to references of one another), regardless of their underlying type:
//!
//! ```
//! use size::Size;
//!
//! let size1 = Size::KiB(4.0 as f64);
//! let size2 = Size::Bytes(4096 as u32);
//! assert_eq!(size1, size2);
//!
//! let size1 = Size::KiB(7);
//! let size2 = Size::KB(7);
//! assert!(&size2 < &size1);
//! ```
//!
//! ## Textual representation
//!
//! The majority of users will be interested in this crate for its ability to "pretty print" sizes
//! with little ceremony and great results. All `Size<T>` instances implement both
//! [`std::fmt::Display`] and [`std::fmt::Debug`], so you can just directly `format!(...)` or
//! `println!(...)` with whatever `Size` you have on hand:
//!
//! ```
//! use size::Size;
//!
//! let file_size = Size::Bytes(1_340_249);
//! let textual = format!("{}", file_size); // "1.28 MiB"
//! assert_eq!(textual.as_str(), "1.28 MiB");
//! ```
//!
//! For fine-grained control over how a size is formatted and displayed, you can manually use the
//! [`Size::to_string()`] function, which accepts parameters that control which units are used
//! ("standard"/base-10 or SI/base-2) and how the unit should be written out, for example:
//!
//! ```
//! use size::{Size, Base, Style};
//!
//! let file_size = Size::Bytes(1_340_249); // same as before
//! let textual_size = format!("{}", file_size.to_string(Base::Base10, Style::FullLowerCase));
//! assert_eq!(textual_size, "1.34 megabytes".to_string());
//! ```
//!
//! ## Mathematical operations
//!
//! You can perform mathematical operations on `Size` types and the type safety makes sure that
//! what you're doing makes sense:
//!
//! ```
//! use size::Size;
//!
//! let sum = Size::MiB(2) + Size::KiB(200);
//! assert_eq!(sum, Size::MB(2.301_952));
//!
//! let size = Size::GB(4.2) / 2;
//! assert_eq!(size, Size::GB(2.1));
//! ```
//!
//! See the documentation of the [`ops`] module for more on this topic.
//!
//! ## Crate features
//!
//! This crate currently has one feature (`std`), enabled by default. If compiled with
//! `--no-default-features` or used as a dependency with default features disabled, the crate
//! becomes `no_std` compatible. When used in `no_std` mode, the following restrictions and
//! limitations are observed:
//!
//! * All formatting/stringification of `Size` types is disabled.
//! * `Size<T>` no longer implements [`std::fmt::Display`] (`core::fmt::Debug` is still
//! implemented).
//! * The intermediate type used for mathematical operations on `Size` types is changed from `f64`
//! to `i64` so that no implicit floating-point math is performed. `Size<f64>` types may still be
//! used, but with the caveat that precision may be lost and truncation (instead of rounding) may be
//! observed when performing mathematical operations on floating-point-backed `Size` types.

pub mod ops;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_nostd;

use core::fmt;
#[cfg(feature = "std")]
use self::Unit::*;
use self::consts::*;
use num_traits::AsPrimitive;

#[cfg(feature = "std")]
type Underlying = f64;
#[cfg(not(feature = "std"))]
type Underlying = i64;

#[cfg(feature = "std")]
const DEFAULT_BASE: Base = Base::Base2;
#[cfg(feature = "std")]
const DEFAULT_STYLE: Style = Style::Smart;

/// A collection of constants for base-2 and base-10 units.
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

/// An enumeration of supported bases to use for generating textual descriptions of sizes.
/// [`Base::Base10`] is the "usual" units like [`Unit::Kilobyte`] and [`Unit::Exabyte`], while
/// [`Base::Base2`] is the SI/memory units like [`Unit::Mebibyte`] and [`Unit::Tebibyte`], (more
/// often referred to as "MiB" and "TiB" respectively).
#[cfg(feature = "std")]
pub enum Base {
    /// Base-2 units like [`Unit::Kibibyte`] and [`Unit::Mebibyte`], more often referred to via
    /// their abbreviations ("KiB" and "MiB", respectively). Each unit is 1024 times greater than
    /// the preceding one.
    Base2,
    /// Base-10 units like [`Unit::Kilobyte`] and [`Unit::Megabyte`]. Each unit is 1000 times
    /// greater than the preceding one.
    Base10,
}

/// A collection of units used to refer to sizes, for all supported bases.
pub enum Unit {
    /// The basic "byte" unit, used by both base-2 and base-10 styles.
    Byte,
    /// The base-2 "kibibyte" unit, equal to 1024 bytes.
    Kibibyte,
    /// The base-10 "kilobyte" unit, equal to 1000 bytes.
    Kilobyte,
    /// The base-2 "mebibyte" unit, equal to 1024 kibibytes.
    Mebibyte,
    /// The base-10 "megabyte" unit, equal to 1000 kilobytes.
    Megabyte,
    /// The base-2 "gibibyte" unit, equal to 1024 mebibytes.
    Gibibyte,
    /// The base-10 "gigabyte" unit, equal to 1000 megabytes.
    Gigabyte,
    /// The base-2 "tebibyte" unit, equal to 1024 gibibytes.
    Tebibyte,
    /// The base-10 "terabyte" unit, equal to 1000 gigabytes.
    Terabyte,
    /// The base-2 "pebibyte" unit, equal to 1024 tebibytes.
    Pebibyte,
    /// The base-10 "petabyte" unit, equal to 1000 terabytes.
    Petabyte,
    /// The base-2 "exbibyte" unit, equal to 1024 pebibytes.
    Exbibyte,
    /// The base-10 "exabyte" unit, equal to 1000 petabytes.
    Exabyte,
}

#[cfg(feature = "std")]
impl Unit {
    fn text(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        match &self {
            &Byte => ("byte", "Byte", "b", "B"),

            &Kilobyte => ("kilobyte", "Kilobyte", "kb", "KB"),
            &Megabyte => ("megabyte", "Megabyte", "mb", "MB"),
            &Gigabyte => ("gigabyte", "Gigabyte", "gb", "GB"),
            &Terabyte => ("terabyte", "Terabyte", "tb", "TB"),
            &Petabyte => ("petabyte", "Petabyte", "pb", "PB"),
            &Exabyte  => ("exabyte",  "Exabyte",  "eb", "EB"),

            &Kibibyte => ("kibibyte", "Kibibyte", "kib", "KiB"),
            &Mebibyte => ("mebibyte", "Mebibyte", "mib", "MiB"),
            &Gibibyte => ("gibibyte", "Gibibyte", "gib", "GiB"),
            &Pebibyte => ("pebibyte", "Pebibyte", "pib", "PiB"),
            &Tebibyte => ("tebibyte", "Tebibyte", "tib", "TiB"),
            &Exbibyte => ("exbibyte", "Exbibyte", "eib", "EiB"),
        }
    }

    fn format(&self, mut fmt: &mut fmt::Formatter, bytes: u64, style: &Style) -> fmt::Result {
        match style {
            Style::Smart => match &self {
                &Unit::Byte => self.format(&mut fmt, bytes, &Style::FullLowerCase),
                _ => self.format(&mut fmt, bytes, &Style::Abbreviated),
            },
            style @ _ => match bytes {
                1 => match style {
                    Style::Smart => unreachable!("already covered above"),
                    Style::FullLowerCase => write!(fmt, " {}", self.text().0),
                    Style::Full => write!(fmt, " {}", self.text().1),
                    Style::AbbreviatedLowerCase => write!(fmt, " {}", self.text().2),
                    Style::Abbreviated => write!(fmt, " {}", self.text().3),
                },
                _ => match style {
                    Style::Smart => unreachable!("already covered above"),
                    Style::FullLowerCase => write!(fmt, " {}s", self.text().0),
                    Style::Full => write!(fmt, " {}s", self.text().1),
                    Style::AbbreviatedLowerCase => write!(fmt, " {}", self.text().2),
                    Style::Abbreviated => write!(fmt, " {}", self.text().3),
                },
            },
        }
    }
}

/// `Size` is the core type exposed by this crate and allows the developer to express the concept of
/// a "size" as a strongly-typed, convertible type that can be used for textual formatting ("pretty
/// printing") and mathematical operations.
///
/// A size is expressed as a unit (any of the enum's discriminant values) and an associated numeric
/// value (the generic `T` parameter). A size is "arbitrarily precise" in the sense that any backing
/// unit may be used and operated on:
///
/// ```
/// use size::Size;
/// // Identical sizes expressed in different units with different backing types:
/// assert_eq!(Size::Kibibytes(2_u8), Size::Kilobytes(2.048_f64));
/// ```
#[derive(Copy, Clone)]
#[non_exhaustive]
pub enum Size<T> {
    /// Express a size in bytes.
    Bytes(T),
    /// Express a size in kibibytes. Actual size is 2^10 \* the value.
    Kibibytes(T),
    /// Express a size in kilobytes. Actual size is 10^3 \* the value.
    Kilobytes(T),
    /// Express a size in kibibytes. Actual size is 2^20 \* the value.
    Mebibytes(T),
    /// Express a size in kilobytes. Actual size is 10^6 \* the value.
    Megabytes(T),
    /// Express a size in kibibytes. Actual size is 2^30 \* the value.
    Gibibytes(T),
    /// Express a size in kilobytes. Actual size is 10^9 \* the value.
    Gigabytes(T),
    /// Express a size in kibibytes. Actual size is 2^40 \* the value.
    Tebibytes(T),
    /// Express a size in kilobytes. Actual size is 10^12 \* the value.
    Terabytes(T),
    /// Express a size in kibibytes. Actual size is 2^50 \* the value.
    Pebibytes(T),
    /// Express a size in kilobytes. Actual size is 10^15 \* the value.
    Petabytes(T),
    /// Express a size in kibibytes. Actual size is 2^60 \* the value.
    Exbibytes(T),
    /// Express a size in kilobytes. Actual size is 10^18 \* the value.
    Exabytes(T),
}

impl<T> Size<T> {
    #![allow(non_snake_case)]

    /// Express a size in bytes, as a shortcut for using [`Size::Bytes`].
    pub const fn B(t: T) -> Self { Self::Bytes(t) }
    /// Express a size in kibibytes, as a shortcut for using [`Size::Kibibytes`].
    pub const fn KiB(t: T) -> Self { Self::Kibibytes(t) }
    /// Express a size in kilobytes, as a shortcut for using [`Size::Kilobytes`].
    pub const fn KB(t: T) -> Self { Self::Kilobytes(t) }
    /// Express a size in mebibytes, as a shortcut for using [`Size::Mebibytes`].
    pub const fn MiB(t: T) -> Self { Self::Mebibytes(t) }
    /// Express a size in megabytes, as a shortcut for using [`Size::Megabytes`].
    pub const fn MB(t: T) -> Self { Self::Megabytes(t) }
    /// Express a size in gibibytes, as a shortcut for using [`Size::Gibibytes`].
    pub const fn GiB(t: T) -> Self { Self::Gibibytes(t) }
    /// Express a size in gigabytes, as a shortcut for using [`Size::Gigabytes`].
    pub const fn GB(t: T) -> Self { Self::Gigabytes(t) }
    /// Express a size in tebibytes, as a shortcut for using [`Size::Tebibytes`].
    pub const fn TiB(t: T) -> Self { Self::Tebibytes(t) }
    /// Express a size in terabytes, as a shortcut for using [`Size::Terabytes`].
    pub const fn TB(t: T) -> Self { Self::Terabytes(t) }
    /// Express a size in pebibytes, as a shortcut for using [`Size::Pebibytes`].
    pub const fn PiB(t: T) -> Self { Self::Pebibytes(t) }
    /// Express a size in petabytes, as a shortcut for using [`Size::Petabytes`].
    pub const fn PB(t: T) -> Self { Self::Petabytes(t) }
    /// Express a size in exbibytes, as a shortcut for using [`Size::Exbibytes`].
    pub const fn EiB(t: T) -> Self { Self::Exbibytes(t) }
    /// Express a size in exabytes, as a shortcut for using [`Size::Exabytes`].
    pub const fn EB(t: T) -> Self { Self::Exabytes(t) }
}

/// An enumeration of supported styles to be used when formatting/printing a [`Size`] type,
/// specifying how the unit should be spelled out.
#[cfg(feature = "std")]
#[non_exhaustive]
pub enum Style {
    /// The default "smart" style, currently equal to [`Style::FullLowerCase`] when the final unit is
    /// in bytes or [`Style::Abbreviated`] otherwise, e.g. "1024 bytes" and "1.29 GiB"
    Smart,
    /// Abbreviated style, e.g. "1024 KB" and "1.29 GiB"
    Abbreviated,
    /// Abbreviated, lowercase style, e.g. "1024 kb" and "1.29 gib"
    AbbreviatedLowerCase,
    /// Full unit name style, e.g. "1024 Kilobytes" and "1.29 Gibibytes"
    Full,
    /// Full, lowercase unit name style, e.g. "1024 kilobytes" and "1.29 gibibytes"
    FullLowerCase,
}

#[cfg(feature = "std")]
impl<T> std::fmt::Display for Size<T>
where
    T: AsPrimitive<f64>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.format(fmt, &DEFAULT_BASE, &DEFAULT_STYLE)
    }
}

impl<T> core::fmt::Debug for Size<T>
where
    T: AsPrimitive<Underlying>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} bytes", self.bytes())
    }
}

impl<T, U> PartialEq<Size<U>> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    fn eq(&self, other: &Size<U>) -> bool {
        self.bytes() == other.bytes()
    }
}

impl<T, U> PartialEq<&Size<U>> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    fn eq(&self, other: &&Size<U>) -> bool {
        self.bytes() == other.bytes()
    }
}

impl<T, U> PartialOrd<Size<U>> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    fn partial_cmp(&self, other: &Size<U>) -> Option<core::cmp::Ordering> {
        self.bytes().partial_cmp(&other.bytes())
    }
}

impl<T, U> PartialOrd<&Size<U>> for Size<T>
where
    T: AsPrimitive<Underlying>,
    U: AsPrimitive<Underlying>,
{
    fn partial_cmp(&self, other: &&Size<U>) -> Option<core::cmp::Ordering> {
        self.bytes().partial_cmp(&other.bytes())
    }
}

struct Fmt<F>(pub F)
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result;

impl<F> fmt::Debug for Fmt<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

impl<T> Size<T>
where
    T: AsPrimitive<Underlying>
{
    /// Calculates the effective size in bytes of the type, useful for obtaining a plain/scalar
    /// representation of the full size represented by a [`Size`] object. This always returns an
    /// `i64` regardless of the underlying type originally used, to avoid (or at least mitigate)
    /// issues with integer overflow (e.g. when trying to retrieve
    /// `Size::Terabyte(16_i32).bytes()`).
    ///
    /// Example:
    /// ```
    /// use size::Size;
    /// assert_eq!(Size::MiB(4_u8).bytes(), 4_194_304 as i64);
    /// ```
    pub fn bytes(&self) -> i64 {
        use self::Size::*;

        (match &self {
            &Bytes(x) => x.as_(),
            &Kilobytes(x) => x.as_() * KILOBYTE as Underlying,
            &Megabytes(x) => x.as_() * MEGABYTE as Underlying,
            &Gigabytes(x) => x.as_() * GIGABYTE as Underlying,
            &Terabytes(x) => x.as_() * TERABYTE as Underlying,
            &Petabytes(x) => x.as_() * PETABYTE as Underlying,
            &Exabytes(x)  => x.as_() * EXABYTE  as Underlying,
            &Kibibytes(x) => x.as_() * KIBIBYTE as Underlying,
            &Mebibytes(x) => x.as_() * MEBIBYTE as Underlying,
            &Gibibytes(x) => x.as_() * GIBIBYTE as Underlying,
            &Tebibytes(x) => x.as_() * TEBIBYTE as Underlying,
            &Pebibytes(x) => x.as_() * PEBIBYTE as Underlying,
            &Exbibytes(x) => x.as_() * EXBIBYTE as Underlying,
        }) as i64
    }

    /// Returns a textual representation of the [`Size`] for display purposes, giving control over
    /// the returned representation's base (see [`Base::Base2`] and [`Base::Base10`]) and the style
    /// used to express the determined unit (see [`Style`]).
    #[cfg(feature = "std")]
    pub fn to_string(&self, base: Base, style: Style) -> String {
        return format!("{:?}", Fmt(|f| self.format(f, &base, &style)));
    }

    #[cfg(feature = "std")]
    fn format(&self, mut fmt: &mut fmt::Formatter, base: &Base, style: &Style) -> fmt::Result {
        let bytes = match self.bytes() {
            x@ 0.. => x as u64,
            y => {
                // TODO: How should this be localized?
                write!(fmt, "-")?;

                // The absolute magnitude of T::min_value() for a signed number is one more than
                // that of T::max_value(), meaning T::min_value().abs() will panic.
                match y.checked_abs() {
                    Some(abs) => abs as u64,
                    None => i64::max_value() as u64,
                }
            }
        };

        let rule = match base {
            Base::Base2 => match BASE2_RULES.binary_search_by_key(&bytes, |rule| rule.less_than) {
                Ok(index) => &BASE2_RULES[index + 1],
                Err(index) => &BASE2_RULES[index],
            },
            Base::Base10 => {
                match BASE10_RULES.binary_search_by_key(&bytes, |rule| rule.less_than) {
                    Ok(index) => &BASE10_RULES[index + 1],
                    Err(index) => &BASE10_RULES[index],
                }
            }
        };

        (rule.formatter)(&mut fmt, bytes)?;
        rule.unit.format(&mut fmt, bytes, &style)?;

        return Ok(());
    }
}

#[cfg(feature = "std")]
struct FormatRule {
    less_than: u64,
    formatter: fn(&mut fmt::Formatter, bytes: u64) -> fmt::Result,
    unit: Unit,
}

#[cfg(feature = "std")]
const BASE10_RULES: [FormatRule; 17] = [
    FormatRule {
        less_than: 1 * KILOBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Byte,
    },
    FormatRule {
        less_than: 10 * KILOBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 100 * KILOBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 1 * MEGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 10 * MEGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 100 * MEGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 1 * GIGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 10 * GIGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 100 * GIGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 1 * TERABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 10 * TERABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 100 * TERABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 1 * PETABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 10 * PETABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 100 * PETABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 1 * EXABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: u64::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1i64 * EXABYTE) as f64)),
        unit: Exabyte,
    },
];

#[cfg(feature = "std")]
const BASE2_RULES: [FormatRule; 17] = [
    FormatRule {
        less_than: 1 * KIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Byte,
    },
    FormatRule {
        less_than: 10 * KIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 100 * KIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 1 * MEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 10 * MEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 100 * MEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 1 * GIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 10 * GIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 100 * GIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 1 * TEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 10 * TEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 100 * TEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 1 * PEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 10 * PEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 100 * PEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 1 * EXBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: u64::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1i64 * EXBIBYTE) as f64)),
        unit: Exbibyte,
    },
];
