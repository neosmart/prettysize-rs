mod ops;
#[cfg(test)]
mod tests;

use self::Unit::*;
use num_traits::ToPrimitive;
use std::fmt;

const DEFAULT_BASE: Base = Base::Base2;
const DEFAULT_STYLE: Style = Style::Smart;

pub const BYTE: u64 = 1;
pub const KILOBYTE: u64 = 1000;
pub const MEGABYTE: u64 = 1000 * KILOBYTE;
pub const GIGABYTE: u64 = 1000 * MEGABYTE;
pub const TERABYTE: u64 = 1000 * GIGABYTE;
pub const PETABYTE: u64 = 1000 * TERABYTE;
pub const EXABYTE: u64 = 1000 * PETABYTE;

pub const B: u64 = BYTE;
pub const KB: u64 = KILOBYTE;
pub const MB: u64 = MEGABYTE;
pub const GB: u64 = GIGABYTE;
pub const TB: u64 = TERABYTE;
pub const PB: u64 = PETABYTE;
pub const EB: u64 = EXABYTE;

pub const KIBIBYTE: u64 = 1 << 10;
pub const MEBIBYTE: u64 = 1 << 20;
pub const GIBIBYTE: u64 = 1 << 30;
pub const TEBIBYTE: u64 = 1 << 40;
pub const PEBIBYTE: u64 = 1 << 50;
pub const EXBIBYTE: u64 = 1 << 60;

#[allow(non_upper_case_globals)]
pub const KiB: u64 = KIBIBYTE;
#[allow(non_upper_case_globals)]
pub const MiB: u64 = MEBIBYTE;
#[allow(non_upper_case_globals)]
pub const GiB: u64 = GIBIBYTE;
#[allow(non_upper_case_globals)]
pub const TiB: u64 = TEBIBYTE;
#[allow(non_upper_case_globals)]
pub const PiB: u64 = PEBIBYTE;
#[allow(non_upper_case_globals)]
pub const EiB: u64 = EXBIBYTE;

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
            &Exabyte => ("exabyte", "Exabyte", "eb", "EB"),

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

pub enum Size<T: ToPrimitive> {
    Bytes(T),
    Kibibytes(T),
    Kilobytes(T),
    Mebibytes(T),
    Megabytes(T),
    Gibibytes(T),
    Gigabytes(T),
    Tebibytes(T),
    Terabytes(T),
    Pebibytes(T),
    Petabytes(T),
    Exbibytes(T),
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
    T: ToPrimitive,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.format(fmt, &DEFAULT_BASE, &DEFAULT_STYLE)
    }
}

impl<T> std::fmt::Debug for Size<T>
where
    T: ToPrimitive,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} bytes", self.bytes())
    }
}

impl<T, U> PartialEq<Size<U>> for Size<T>
where
    T: ToPrimitive,
    U: ToPrimitive,
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
    T: ToPrimitive,
{
    pub fn bytes(&self) -> u64 {
        use self::Size::*;

        match &self {
            &Bytes(x) => x.to_f64().unwrap(),
            &Kilobytes(x) => x.to_f64().unwrap() * KILOBYTE as f64,
            &Megabytes(x) => x.to_f64().unwrap() * MEGABYTE as f64,
            &Gigabytes(x) => x.to_f64().unwrap() * GIGABYTE as f64,
            &Terabytes(x) => x.to_f64().unwrap() * TERABYTE as f64,
            &Petabytes(x) => x.to_f64().unwrap() * PETABYTE as f64,
            &Exabytes(x) => x.to_f64().unwrap() * EXABYTE as f64,
            &Kibibytes(x) => x.to_f64().unwrap() * KIBIBYTE as f64,
            &Mebibytes(x) => x.to_f64().unwrap() * MEBIBYTE as f64,
            &Gibibytes(x) => x.to_f64().unwrap() * GIBIBYTE as f64,
            &Tebibytes(x) => x.to_f64().unwrap() * TEBIBYTE as f64,
            &Pebibytes(x) => x.to_f64().unwrap() * PEBIBYTE as f64,
            &Exbibytes(x) => x.to_f64().unwrap() * EXBIBYTE as f64,
        }
        .to_u64()
        .unwrap()
    }

    pub fn to_string(&self, base: Base, style: Style) -> String {
        return format!("{:?}", Fmt(|f| self.format(f, &base, &style)));
    }

    fn format(&self, mut fmt: &mut fmt::Formatter, base: &Base, style: &Style) -> fmt::Result {
        let bytes = self.bytes();

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

struct FormatRule {
    less_than: u64,
    formatter: fn(&mut fmt::Formatter, bytes: u64) -> fmt::Result,
    unit: Unit,
}

const BASE10_RULES: [FormatRule; 19] = [
    FormatRule {
        less_than: 0,
        formatter: |_, _| panic!("u64 less than zero!"),
        unit: Byte,
    },
    FormatRule {
        less_than: 1 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Byte,
    },
    FormatRule {
        less_than: 10 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 100 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 1 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 10 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 100 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 1 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 10 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 100 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 1 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 10 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 100 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 1 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 10 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 100 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 1 * EXABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 10 * EXABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * EXABYTE) as f64)),
        unit: Exabyte,
    },
    FormatRule {
        less_than: u64::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1u64 * EXABYTE) as f64)),
        unit: Exabyte,
    },
];

const BASE2_RULES: [FormatRule; 19] = [
    FormatRule {
        less_than: 0,
        formatter: |_, _| panic!("u64 less than zero!"),
        unit: Byte,
    },
    FormatRule {
        less_than: 1 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Byte,
    },
    FormatRule {
        less_than: 10 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 100 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 1 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 10 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 100 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 1 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 10 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 100 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 1 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 10 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 100 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 1 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 10 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 100 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1u64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 1 * EXBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1u64 * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 10 * EXBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1u64 * EXBIBYTE) as f64)),
        unit: Exbibyte,
    },
    FormatRule {
        less_than: u64::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1u64 * EXBIBYTE) as f64)),
        unit: Exbibyte,
    },
];
