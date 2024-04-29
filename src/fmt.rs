//! The `fmt` module contains [`SizeFormatter`] and other types pertaining to formatting a size as
//! human-readable text.
//!
//! You will likely not need to interact with this module directly, as the functionality of a
//! [`SizeFormatter`] is exposed by simply calling [`Size::format()`]. However, a new
//! [`SizeFormatter`] can be instantiated directly if you would like a standalone pretty-printer for
//! raw byte sizes.
//!
//! The formatting-related enums in this module ([`Base`] and [`Style`]) are re-exported at the
//! crate level as `size::Base` and `size::Style`.

use super::*;
use core::fmt;

/// An enumeration of supported bases to use for generating textual descriptions of sizes.
///
/// [`Base::Base10`] is the "usual" units like "kilobyte" and "exabyte", while [`Base::Base2`] is
/// the SI/memory units like "mebibyte" and "tebibyte", (more often referred to as "MiB" and "TiB",
/// respectively).
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Base {
    /// Base-2 units like "kibibyte" and "mebibyte", more often referred to via their abbreviations
    /// ("KiB" and "MiB", respectively). Each unit is 1024 times greater than the preceding one.
    Base2,
    /// Base-10 units like "kilobyte" and "megabyte". Each unit is 1000 times greater than the
    /// preceding one.
    Base10,
}

/// A collection of units used to refer to sizes, for all supported bases.
enum Unit {
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

impl Unit {
    #[rustfmt::skip]
    const fn text(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        use self::Unit::*;

        match self {
            Byte => ("byte", "Byte", "b", "B"),

            Kilobyte => ("kilobyte", "Kilobyte", "kb", "KB"),
            Megabyte => ("megabyte", "Megabyte", "mb", "MB"),
            Gigabyte => ("gigabyte", "Gigabyte", "gb", "GB"),
            Terabyte => ("terabyte", "Terabyte", "tb", "TB"),
            Petabyte => ("petabyte", "Petabyte", "pb", "PB"),
            Exabyte  => ("exabyte",  "Exabyte",  "eb", "EB"),

            Kibibyte => ("kibibyte", "Kibibyte", "kib", "KiB"),
            Mebibyte => ("mebibyte", "Mebibyte", "mib", "MiB"),
            Gibibyte => ("gibibyte", "Gibibyte", "gib", "GiB"),
            Pebibyte => ("pebibyte", "Pebibyte", "pib", "PiB"),
            Tebibyte => ("tebibyte", "Tebibyte", "tib", "TiB"),
            Exbibyte => ("exbibyte", "Exbibyte", "eib", "EiB"),
        }
    }

    fn format(&self, fmt: &mut fmt::Formatter, bytes: u64, style: &Style) -> fmt::Result {
        match (&style, bytes) {
            (&Style::Default, _) => match &self {
                &Unit::Byte => self.format(fmt, bytes, &Style::FullLowercase),
                _ => self.format(fmt, bytes, &Style::Abbreviated),
            },

            (&Style::FullLowercase, 1) => write!(fmt, " {}", self.text().0),
            (&Style::Full, 1) => write!(fmt, " {}", self.text().1),
            (&Style::AbbreviatedLowercase, 1) => write!(fmt, " {}", self.text().2),
            (&Style::Abbreviated, 1) => write!(fmt, " {}", self.text().3),

            (&Style::FullLowercase, _) => write!(fmt, " {}s", self.text().0),
            (&Style::Full, _) => write!(fmt, " {}s", self.text().1),
            (&Style::AbbreviatedLowercase, _) => write!(fmt, " {}", self.text().2),
            (&Style::Abbreviated, _) => write!(fmt, " {}", self.text().3),
        }
    }
}

/// An enumeration of supported styles to be used when formatting/printing a [`Size`] type,
/// specifying how the unit should be spelled out.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Style {
    /// The default "smart" style, currently equal to [`Style::FullLowercase`] when the final unit
    /// is in bytes or [`Style::Abbreviated`] otherwise, e.g. "1024 bytes" and "1.29 GiB"
    Default,
    /// Abbreviated style, e.g. "1024 KB" and "1.29 GiB"
    Abbreviated,
    /// Abbreviated, lowercase style, e.g. "1024 kb" and "1.29 gib"
    AbbreviatedLowercase,
    /// Full unit name style, e.g. "1024 Kilobytes" and "1.29 Gibibytes"
    Full,
    /// Full, lowercase unit name style, e.g. "1024 kilobytes" and "1.29 gibibytes"
    FullLowercase,
}

// Backwards-compatibility associated constants to mimic `Style` variants to enable compilation of
// older code. They are all hidden from the docs.
impl Style {
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.3.0", note = "Use Style::Default instead")]
    /// A backwards-compatible alias for [`Style::Default`]
    pub const Smart: Style = Style::Default;

    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.3.0", note = "Use Style::AbbreviatedLowercase instead")]
    /// A backwards-compatible alias for [`Style::AbbreviatedLowercase`]
    pub const AbbreviatedLowerCase: Style = Style::AbbreviatedLowercase;

    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.3.0", note = "Use Style::FullLowercase instead")]
    /// A backwards-compatible alias for [`Style::FullLowercase`]
    pub const FullLowerCase: Style = Style::FullLowercase;
}

impl std::fmt::Display for Size {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.format())
    }
}

mod sealed {
    pub trait FormatterSize {}

    impl FormatterSize for () {}
    impl<'a> FormatterSize for &'a crate::Size {}
}

/// A standalone size formatter that is configured via the builder pattern (via the various `.with_`
/// methods) which can then be used to format an integral byte value as a pretty printed `String` in
/// accordance with the configured properties.
///
/// Use of the strongly typed [`Size`] to hold and display sizes is strongly preferred over this
/// approach, but it may come in handy when you have many sizes and all need to be formatted in an
/// identical and manually-specified fashion.
///
/// ```
/// use size::{Base, Size, SizeFormatter, Style};
///
/// let formatter = SizeFormatter::new()
///     .with_base(Base::Base10)
///     .with_style(Style::Abbreviated);
///
/// # let mut sizes: Vec<String> = Vec::new();
/// for raw_size in [ 1024, 2048, 4096 ]
/// #   // Work around limitation in earlier rustc versions (e.g. 1.50)
/// #   .iter()
/// {
/// #   // Work around limitation in earlier rustc versions (e.g. 1.50)
/// #   let raw_size = *raw_size;
///     let formatted = formatter.format(raw_size);
///     println!("{}", &formatted);
/// #   sizes.push(formatted);
/// }
///
/// // Prints:
/// // 1.02 KB
/// // 2.05 KB
/// // 4.10 KB
///
/// # assert_eq!(sizes[0].as_str(), "1.02 KB");
/// # assert_eq!(sizes[1].as_str(), "2.05 KB");
/// # assert_eq!(sizes[2].as_str(), "4.10 KB");
/// ```
pub struct SizeFormatter<T: sealed::FormatterSize = ()> {
    size: T,
    base: Base,
    style: Style,
    scale: Option<usize>,
}

/// Makes it possible to obtain a string from an `fmt(f: &mut Formatter)` function by initializing
/// this type as a wrapper around said format function, then using `format!("{}", foo)` on the
/// resulting object.
struct FmtRenderer<F: Fn(&mut fmt::Formatter) -> fmt::Result> {
    formatter: F,
}

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> FmtRenderer<F> {
    pub fn new(formatter: F) -> Self {
        Self { formatter }
    }
}

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for FmtRenderer<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.formatter)(f)
    }
}

impl<T: sealed::FormatterSize> SizeFormatter<T> {
    /// Specify the base of the units to be used when generating the textual description of the
    /// `Size`.
    ///
    /// This lets users choose between "standard" base-10 units like "KB" and "MB" or the improved
    /// SI base-2 units like "KiB" and "MiB". See [`Base`] for more information.
    pub fn with_base(self, base: Base) -> Self {
        Self { base, ..self }
    }

    /// Specify the style used to write the accompanying unit for a formatted file size.
    ///
    /// See [`Style`] for more information.
    pub fn with_style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    /// Formats the provided `bytes` value with the configured [`self.Base`] and [`self.Style`].
    fn inner_fmt(&self, fmt: &mut fmt::Formatter, bytes: i64) -> fmt::Result {
        let bytes = match bytes {
            x @ 0..=i64::MAX => x as u64,
            y => {
                write!(fmt, "-")?;

                // The absolute magnitude of T::min_value() for a signed number is one more than
                // that of T::max_value(), meaning T::min_value().abs() will panic.
                match y.checked_abs() {
                    Some(abs) => abs as u64,
                    None => i64::max_value() as u64,
                }
            }
        };

        let rule = match self.base {
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

        (rule.formatter)(fmt, bytes, self.scale)?;
        rule.unit.format(fmt, bytes, &self.style)?;

        Ok(())
    }
}

impl SizeFormatter<()> {
    /// Create a new `SizeFormatter` that can be used to repeatedly format a number of file sizes
    /// according to its configured options.
    pub const fn new() -> SizeFormatter<()> {
        SizeFormatter {
            size: (),
            base: DEFAULT_BASE,
            style: DEFAULT_STYLE,
            scale: DEFAULT_SCALE,
        }
    }

    /// Formats a provided size in bytes as a string, per the configuration of the current
    /// `SizeFormatter` instance.
    pub fn format(&self, bytes: i64) -> String {
        format!(
            "{}",
            FmtRenderer::new(|fmt: &mut fmt::Formatter| { self.inner_fmt(fmt, bytes) })
        )
    }
}

/// A type that can be used to achieve greater control over how a [`Size`] is formatted as
/// human-readable text, created by calling [`Size::format()`]. The `SizeFormatter` follows the
/// builder model and exposes a chaining API for configuration (via the `.with_` functions).
///
/// After configuration, a `FormattableSize` may be passed directly to the `println!()` or
/// `format!()` macros and their friends because it implements [`Display`](std::fmt::Display), or
/// [`FormattableSize::to_string()`](ToString::to_string) can be used to retrieve a `String`
/// containing the formatted result.
///
/// Example:
/// ```
/// use size::{Base, Size, Style};
///
/// let size = Size::from_mib(1.907349);
/// let text = size.format()
///     .with_base(Base::Base10)
///     .with_style(Style::Full)
///     .to_string();
///
/// assert_eq!(text.as_str(), "2.00 Megabytes");
/// ```
pub type FormattableSize<'a> = SizeFormatter<&'a Size>;

impl fmt::Display for FormattableSize<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner_fmt(f, self.size.bytes())
    }
}

impl Size {
    /// Returns a textual representation of the [`Size`] for display purposes, giving control over
    /// the returned representation's base (see [`Base::Base2`] and [`Base::Base10`]) and the style
    /// used to express the determined unit (see [`Style`]).
    ///
    /// Example:
    /// ```
    /// use size::{Base, Size, Style};
    ///
    /// let size = Size::from_mib(1.907349);
    /// let text = size.format()
    ///     .with_base(Base::Base10)
    ///     .with_style(Style::Full)
    ///     .to_string();
    ///
    /// assert_eq!(text.as_str(), "2.00 Megabytes");
    /// ```
    ///
    /// It is not necessary to call `.to_string()` if you are passing the formatted size to a
    /// `format!()` macro or similar (e.g. `println!` and friends), as the result implements
    /// [`Display`](std::fmt::Display) and will resolve to the same text.
    pub fn format(&self) -> FormattableSize {
        FormattableSize {
            size: self,
            base: DEFAULT_BASE,
            style: DEFAULT_STYLE,
            scale: DEFAULT_SCALE,
        }
    }
}

struct FormatRule {
    less_than: u64,
    formatter: fn(&mut fmt::Formatter, bytes: u64, scale: Option<usize>) -> fmt::Result,
    unit: Unit,
}

const BASE10_RULES: [FormatRule; 17] = [
    FormatRule {
        less_than: KILOBYTE as u64,
        formatter: |fmt, bytes, _| write!(fmt, "{0:.0}", bytes),
        unit: Unit::Byte,
    },
    FormatRule {
        less_than: 10 * KILOBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (KILOBYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Kilobyte,
    },
    FormatRule {
        less_than: 100 * KILOBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (KILOBYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Kilobyte,
    },
    FormatRule {
        less_than: MEGABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (KILOBYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Kilobyte,
    },
    FormatRule {
        less_than: 10 * MEGABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (MEGABYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Megabyte,
    },
    FormatRule {
        less_than: 100 * MEGABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (MEGABYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Megabyte,
    },
    FormatRule {
        less_than: GIGABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (MEGABYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Megabyte,
    },
    FormatRule {
        less_than: 10 * GIGABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (GIGABYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Gigabyte,
    },
    FormatRule {
        less_than: 100 * GIGABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (GIGABYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Gigabyte,
    },
    FormatRule {
        less_than: TERABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (GIGABYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Gigabyte,
    },
    FormatRule {
        less_than: 10 * TERABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (TERABYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Terabyte,
    },
    FormatRule {
        less_than: 100 * TERABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (TERABYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Terabyte,
    },
    FormatRule {
        less_than: PETABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (TERABYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Terabyte,
    },
    FormatRule {
        less_than: 10 * PETABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (PETABYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Petabyte,
    },
    FormatRule {
        less_than: 100 * PETABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (PETABYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Petabyte,
    },
    FormatRule {
        less_than: EXABYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (PETABYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Petabyte,
    },
    FormatRule {
        less_than: u64::max_value(),
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (EXABYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Exabyte,
    },
];

const BASE2_RULES: [FormatRule; 17] = [
    FormatRule {
        less_than: KIBIBYTE as u64,
        formatter: |fmt, bytes, _| write!(fmt, "{0:.0}", bytes),
        unit: Unit::Byte,
    },
    FormatRule {
        less_than: 10 * KIBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (KIBIBYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Kibibyte,
    },
    FormatRule {
        less_than: 100 * KIBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (KIBIBYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Kibibyte,
    },
    FormatRule {
        less_than: MEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (KIBIBYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Kibibyte,
    },
    FormatRule {
        less_than: 10 * MEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (MEBIBYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Mebibyte,
    },
    FormatRule {
        less_than: 100 * MEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (MEBIBYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Mebibyte,
    },
    FormatRule {
        less_than: GIBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (MEBIBYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Mebibyte,
    },
    FormatRule {
        less_than: 10 * GIBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (GIBIBYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Gibibyte,
    },
    FormatRule {
        less_than: 100 * GIBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (GIBIBYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Gibibyte,
    },
    FormatRule {
        less_than: TEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (GIBIBYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Gibibyte,
    },
    FormatRule {
        less_than: 10 * TEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (TEBIBYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Tebibyte,
    },
    FormatRule {
        less_than: 100 * TEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (TEBIBYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Tebibyte,
    },
    FormatRule {
        less_than: PEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (TEBIBYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Tebibyte,
    },
    FormatRule {
        less_than: 10 * PEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (PEBIBYTE as f64), scale.unwrap_or(2))
        },
        unit: Unit::Pebibyte,
    },
    FormatRule {
        less_than: 100 * PEBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (PEBIBYTE as f64), scale.unwrap_or(1))
        },
        unit: Unit::Pebibyte,
    },
    FormatRule {
        less_than: EXBIBYTE as u64,
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (PEBIBYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Pebibyte,
    },
    FormatRule {
        less_than: u64::max_value(),
        formatter: |fmt, bytes, scale| {
            write!(fmt, "{0:.1$}", bytes as f64 / (EXBIBYTE as f64), scale.unwrap_or(0))
        },
        unit: Unit::Exbibyte,
    },
];
