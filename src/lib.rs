mod ops;
#[cfg(test)]
mod tests;

use self::Unit::*;
use num_traits::AsPrimitive;
use std::fmt;

const DEFAULT_BASE: Base = Base::Base2;
const DEFAULT_STYLE: Style = Style::Smart;

pub const BYTE: i64 = 1;
pub const KILOBYTE: i64 = 1000;
pub const MEGABYTE: i64 = 1000 * KILOBYTE;
pub const GIGABYTE: i64 = 1000 * MEGABYTE;
pub const TERABYTE: i64 = 1000 * GIGABYTE;
pub const PETABYTE: i64 = 1000 * TERABYTE;
pub const EXABYTE: i64 = 1000 * PETABYTE;

pub const B: i64 = BYTE;
pub const KB: i64 = KILOBYTE;
pub const MB: i64 = MEGABYTE;
pub const GB: i64 = GIGABYTE;
pub const TB: i64 = TERABYTE;
pub const PB: i64 = PETABYTE;
pub const EB: i64 = EXABYTE;

pub const KIBIBYTE: i64 = 1 << 10;
pub const MEBIBYTE: i64 = 1 << 20;
pub const GIBIBYTE: i64 = 1 << 30;
pub const TEBIBYTE: i64 = 1 << 40;
pub const PEBIBYTE: i64 = 1 << 50;
pub const EXBIBYTE: i64 = 1 << 60;

#[allow(non_upper_case_globals)]
pub const KiB: i64 = KIBIBYTE;
#[allow(non_upper_case_globals)]
pub const MiB: i64 = MEBIBYTE;
#[allow(non_upper_case_globals)]
pub const GiB: i64 = GIBIBYTE;
#[allow(non_upper_case_globals)]
pub const TiB: i64 = TEBIBYTE;
#[allow(non_upper_case_globals)]
pub const PiB: i64 = PEBIBYTE;
#[allow(non_upper_case_globals)]
pub const EiB: i64 = EXBIBYTE;

pub enum Base {
    Base2,
    Base10,
}

pub enum Unit {
    Byte,
    Kibibyte,
    Kilobyte,
    Mebibyte,
    Megabyte,
    Gibibyte,
    Gigabyte,
    Tebibyte,
    Terabyte,
    Pebibyte,
    Petabyte,
    Exbibyte,
    Exabyte,
}

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

    fn format(&self, mut fmt: &mut fmt::Formatter, bytes: i64, style: &Style) -> fmt::Result {
        match style {
            Style::Smart => match &self {
                &Unit::Byte => self.format(&mut fmt, bytes, &Style::FullLowerCase),
                _ => self.format(&mut fmt, bytes, &Style::Abbreviated),
            },
            style @ _ => match bytes {
                1 => match style {
                    Style::Smart => panic!("already covered above"),
                    Style::FullLowerCase => write!(fmt, " {}", self.text().0),
                    Style::Full => write!(fmt, " {}", self.text().1),
                    Style::AbbreviatedLowerCase => write!(fmt, " {}", self.text().2),
                    Style::Abbreviated => write!(fmt, " {}", self.text().3),
                },
                _ => match style {
                    Style::Smart => panic!("already covered above"),
                    Style::FullLowerCase => write!(fmt, " {}s", self.text().0),
                    Style::Full => write!(fmt, " {}s", self.text().1),
                    Style::AbbreviatedLowerCase => write!(fmt, " {}", self.text().2),
                    Style::Abbreviated => write!(fmt, " {}", self.text().3),
                },
            },
        }
    }
}

#[derive(Copy, Clone)]
pub enum Size<T> {
    B(T),
    Bytes(T),
    KiB(T),
    Kibibytes(T),
    KB(T),
    Kilobytes(T),
    MiB(T),
    Mebibytes(T),
    MB(T),
    Megabytes(T),
    GiB(T),
    Gibibytes(T),
    GB(T),
    Gigabytes(T),
    TiB(T),
    Tebibytes(T),
    TB(T),
    Terabytes(T),
    PiB(T),
    Pebibytes(T),
    PB(T),
    Petabytes(T),
    EiB(T),
    Exbibytes(T),
    EB(T),
    Exabytes(T),
}

pub enum Style {
    Abbreviated,
    AbbreviatedLowerCase,
    Full,
    Smart,
    FullLowerCase,
}

impl<T> std::fmt::Display for Size<T>
where
    T: AsPrimitive<f64>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.format(fmt, &DEFAULT_BASE, &DEFAULT_STYLE)
    }
}

impl<T> std::fmt::Debug for Size<T>
where
    T: AsPrimitive<f64>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} bytes", self.bytes())
    }
}

impl<T, U> PartialEq<Size<U>> for Size<T>
where
    T: AsPrimitive<f64>,
    U: AsPrimitive<f64>,
{
    fn eq(&self, other: &Size<U>) -> bool {
        self.bytes() == other.bytes()
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
    T: AsPrimitive<f64>
{
    pub fn bytes(&self) -> i64 {
        use self::Size::*;

        (match &self {
            &Bytes(x) | &B(x) => x.as_(),
            &Kilobytes(x) | &KB(x) => x.as_() * KILOBYTE as f64,
            &Megabytes(x) | &MB(x) => x.as_() * MEGABYTE as f64,
            &Gigabytes(x) | &GB(x) => x.as_() * GIGABYTE as f64,
            &Terabytes(x) | &TB(x) => x.as_() * TERABYTE as f64,
            &Petabytes(x) | &PB(x) => x.as_() * PETABYTE as f64,
            &Exabytes(x)  | &EB(x) => x.as_() * EXABYTE  as f64,
            &Kibibytes(x) | &KiB(x) => x.as_() * KIBIBYTE as f64,
            &Mebibytes(x) | &MiB(x) => x.as_() * MEBIBYTE as f64,
            &Gibibytes(x) | &GiB(x) => x.as_() * GIBIBYTE as f64,
            &Tebibytes(x) | &TiB(x) => x.as_() * TEBIBYTE as f64,
            &Pebibytes(x) | &PiB(x) => x.as_() * PEBIBYTE as f64,
            &Exbibytes(x) | &EiB(x) => x.as_() * EXBIBYTE as f64,
        }) as i64
    }

    pub fn to_string(&self, base: Base, style: Style) -> String {
        return format!("{:?}", Fmt(|f| self.format(f, &base, &style)));
    }

    fn format(&self, mut fmt: &mut fmt::Formatter, base: &Base, style: &Style) -> fmt::Result {
        let bytes = match self.bytes() {
            x@ 0.. => x,
            y => {
                write!(fmt, "-")?;

                // The absolute magnitude of T::min_value() for a signed number is one more than
                // that of T::max_value(), meaning T::min_value().abs() will panic.
                match y.checked_abs() {
                    Some(abs) => abs,
                    None => i64::max_value(),
                }
            }
        };

        const BASE2_TEMP: usize = BASE2_RULES.len() - 1;
        const BASE10_TEMP: usize = BASE10_RULES.len() - 1;
        let rule = match base {
            Base::Base2 => match BASE2_RULES.binary_search_by_key(&bytes, |rule| rule.less_than) {
                Ok(BASE2_TEMP) => &BASE2_RULES[BASE2_RULES.len() - 1],
                Ok(index) => &BASE2_RULES[index + 1],
                Err(index) => &BASE2_RULES[index],
            },
            Base::Base10 => {
                match BASE10_RULES.binary_search_by_key(&bytes, |rule| rule.less_than) {
                    Ok(BASE10_TEMP) => &BASE10_RULES[BASE10_RULES.len() - 1],
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

struct FormatRule {
    less_than: i64,
    formatter: fn(&mut fmt::Formatter, bytes: i64) -> fmt::Result,
    unit: Unit,
}

const BASE10_RULES: [FormatRule; 18] = [
    FormatRule {
        less_than: 0,
        formatter: |_, _| unreachable!("format for less than zero!"),
        unit: Byte,
    },
    FormatRule {
        less_than: 1 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Byte,
    },
    FormatRule {
        less_than: 10 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 100 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 1 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 10 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 100 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 1 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 10 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 100 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 1 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 10 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 100 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 1 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 10 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 100 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 1 * EXABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: i64::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1i64 * EXABYTE) as f64)),
        unit: Exabyte,
    },
];

const BASE2_RULES: [FormatRule; 18] = [
    FormatRule {
        less_than: 0,
        formatter: |_, _| unreachable!("format for less than zero!"),
        unit: Byte,
    },
    FormatRule {
        less_than: 1 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Byte,
    },
    FormatRule {
        less_than: 10 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 100 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 1 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 10 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 100 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 1 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 10 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 100 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 1 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 10 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 100 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 1 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 10 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 100 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 1 * EXBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: i64::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1i64 * EXBIBYTE) as f64)),
        unit: Exbibyte,
    },
];
