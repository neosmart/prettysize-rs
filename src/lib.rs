extern crate num_traits;

use num_traits::ToPrimitive;
use std::fmt;
use Units::*;

#[cfg(test)]
mod tests;

const DEFAULT_BASE: Base = Base::Base2;
const DEFAULT_STYLE: Style = Style::Smart;

pub const BYTE:     usize = 1;
pub const KILOBYTE: usize = 1000;
pub const MEGABYTE: usize = 1000 * KILOBYTE;
pub const GIGABYTE: usize = 1000 * MEGABYTE;
pub const TERABYTE: usize = 1000 * MEGABYTE;
pub const PETABYTE: usize = 1000 * TERABYTE;
pub const EXABYTE:  usize = 1000 * PETABYTE;

pub const B:  usize = BYTE;
pub const KB: usize = KILOBYTE;
pub const MB: usize = MEGABYTE;
pub const GB: usize = GIGABYTE;
pub const TB: usize = TERABYTE;
pub const PB: usize = PETABYTE;
pub const EB: usize = EXABYTE;

pub const KIBIBYTE: usize = 1 << 10;
pub const MEBIBYTE: usize = 1 << 20;
pub const GIBIBYTE: usize = 1 << 30;
pub const TEBIBYTE: usize = 1 << 40;
pub const PEBIBYTE: usize = 1 << 50;
pub const EXBIBYTE: usize = 1 << 60;

#[allow(non_upper_case_globals)]
pub const KiB: usize = KIBIBYTE;
#[allow(non_upper_case_globals)]
pub const MiB: usize = MEBIBYTE;
#[allow(non_upper_case_globals)]
pub const GiB: usize = GIBIBYTE;
#[allow(non_upper_case_globals)]
pub const TiB: usize = TEBIBYTE;
#[allow(non_upper_case_globals)]
pub const PiB: usize = PEBIBYTE;
#[allow(non_upper_case_globals)]
pub const EiB: usize = EXBIBYTE;

pub enum Base {
    Base2,
    Base10,
}

pub enum Units {
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

impl Units {
    fn text(&self)
        -> (&'static str, &'static str, &'static str, &'static str, &'static str, &'static str)
    {
        match &self {
            &Byte => ("byte", "bytes", "Byte", "Bytes", "b", "B"),

            &Kilobyte => ("kilobyte", "kilobytes", "Kilobyte", "Kilobytes", "kb", "KB"),
            &Megabyte => ("megabyte", "megabytes", "Megabyte", "Megabytes", "mb", "MB"),
            &Gigabyte => ("gigabyte", "gigabytes", "Gigabyte", "Gigabytes", "gb", "GB"),
            &Terabyte => ("terabyte", "terabytes", "Terabyte", "Terabytes", "tb", "TB"),
            &Petabyte => ("petabyte", "petabytes", "Petabyte", "Petabytes", "pb", "PB"),
            &Exabyte =>  ("exabyte",  "exabytes",  "Exabyte",  "Exabytes",  "eb", "EB"),

            &Kibibyte => ("kibibyte", "kibibytes", "Kibibyte", "Kibibytes", "kib", "KiB",),
            &Mebibyte => ("mebibyte", "mebibytes", "Mebibyte", "Mebibytes", "mib", "MiB",),
            &Gibibyte => ("gibibyte", "gibibytes", "Gibibyte", "Gibibytes", "gib", "GiB",),
            &Pebibyte => ("pebibyte", "pebibytes", "Pebibyte", "Pebibytes", "pib", "PiB",),
            &Tebibyte => ("tebibyte", "tebibytes", "Tebibyte", "Tebibytes", "tib", "TiB",),
            &Exbibyte => ("exbibyte", "exbibytes", "Exbibyte", "Exbibytes", "eib", "EiB",),
        }
    }

    fn format(&self, mut fmt: &mut fmt::Formatter, bytes: usize, style: &Style) -> fmt::Result {
        match style {
            Style::Smart => match &self {
                &Units::Byte => self.format(&mut fmt, bytes, &Style::FullLowerCase),
                _ => self.format(&mut fmt, bytes, &Style::Abbreviated),
            },
            style @ _ => match bytes {
                1 => match style {
                    Style::Smart => panic!("already covered above"),
                    Style::FullLowerCase => write!(fmt, " {}", self.text().0),
                    Style::Full => write!(fmt, " {}", self.text().2),
                    Style::AbbreviatedLowerCase => write!(fmt, " {}", self.text().4),
                    Style::Abbreviated => write!(fmt, " {}", self.text().5),
                },
                _ => match style {
                    Style::Smart => panic!("already covered above"),
                    Style::FullLowerCase => write!(fmt, " {}", self.text().1),
                    Style::Full => write!(fmt, " {}", self.text().3),
                    Style::AbbreviatedLowerCase => write!(fmt, " {}", self.text().4),
                    Style::Abbreviated => write!(fmt, " {}", self.text().5),
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
        self.format(fmt, DEFAULT_BASE, DEFAULT_STYLE)
    }
}

impl<T> Size<T>
where
    T: ToPrimitive,
{
    fn bytes(&self) -> usize {
        use Size::*;

        match &self {
            &Bytes(x) => x.to_f64().unwrap(),
            &Kilobytes(x) => x.to_f64().unwrap() * KILOBYTE as f64,
            &Megabytes(x) => x.to_f64().unwrap() * MEGABYTE as f64,
            &Gigabytes(x) => x.to_f64().unwrap() * GIGABYTE as f64,
            &Terabytes(x) => x.to_f64().unwrap() * TERABYTE as f64,
            &Petabytes(x) => x.to_f64().unwrap() * PETABYTE as f64,
            &Exabytes(x)  => x.to_f64().unwrap() * EXABYTE  as f64,
            &Kibibytes(x) => x.to_f64().unwrap() * KIBIBYTE as f64,
            &Mebibytes(x) => x.to_f64().unwrap() * MEBIBYTE as f64,
            &Gibibytes(x) => x.to_f64().unwrap() * GIBIBYTE as f64,
            &Tebibytes(x) => x.to_f64().unwrap() * TEBIBYTE as f64,
            &Pebibytes(x) => x.to_f64().unwrap() * PEBIBYTE as f64,
            &Exbibytes(x) => x.to_f64().unwrap() * EXBIBYTE as f64,
        }.to_usize()
        .unwrap()
    }

    fn format(&self, mut fmt: &mut fmt::Formatter, base: Base, style: Style) -> fmt::Result {
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
    less_than: usize,
    formatter: fn(&mut fmt::Formatter, bytes: usize) -> fmt::Result,
    unit: Units,
}

const BASE10_RULES: [FormatRule; 20] = [
    FormatRule {
        less_than: 0,
        formatter: |_, _| panic!("usize less than zero!"),
        unit: Byte,
    },
    FormatRule {
        less_than: 1 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Byte,
    },
    FormatRule {
        less_than: 10 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 100 * KILOBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 1 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * KILOBYTE) as f64)),
        unit: Kilobyte,
    },
    FormatRule {
        less_than: 10 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 100 * MEGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 1 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * MEGABYTE) as f64)),
        unit: Megabyte,
    },
    FormatRule {
        less_than: 10 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 100 * GIGABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 1 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * GIGABYTE) as f64)),
        unit: Gigabyte,
    },
    FormatRule {
        less_than: 10 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 100 * TERABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 1 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * TERABYTE) as f64)),
        unit: Terabyte,
    },
    FormatRule {
        less_than: 10 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 100 * PETABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 1 * EXABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * PETABYTE) as f64)),
        unit: Petabyte,
    },
    FormatRule {
        less_than: 10 * EXABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * EXABYTE) as f64)),
        unit: Exabyte,
    },
    FormatRule {
        less_than: 100 * EXABYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * EXABYTE) as f64)),
        unit: Exabyte,
    },
    FormatRule {
        less_than: usize::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1usize * EXABYTE) as f64)),
        unit: Exabyte,
    },
];

const BASE2_RULES: [FormatRule; 19] = [
    FormatRule {
        less_than: 0,
        formatter: |_, _| panic!("usize less than zero!"),
        unit: Byte,
    },
    FormatRule {
        less_than: 1 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Byte,
    },
    FormatRule {
        less_than: 10 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 100 * KIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 1 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * KIBIBYTE) as f64)),
        unit: Kibibyte,
    },
    FormatRule {
        less_than: 10 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 100 * MEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 1 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * MEBIBYTE) as f64)),
        unit: Mebibyte,
    },
    FormatRule {
        less_than: 10 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 100 * GIBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 1 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * GIBIBYTE) as f64)),
        unit: Gibibyte,
    },
    FormatRule {
        less_than: 10 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 100 * TEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 1 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * TEBIBYTE) as f64)),
        unit: Tebibyte,
    },
    FormatRule {
        less_than: 10 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 100 * PEBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1usize * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 1 * EXBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1usize * PEBIBYTE) as f64)),
        unit: Pebibyte,
    },
    FormatRule {
        less_than: 10 * EXBIBYTE,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1usize * EXBIBYTE) as f64)),
        unit: Exbibyte,
    },
    FormatRule {
        less_than: usize::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1usize * EXBIBYTE) as f64)),
        unit: Exbibyte,
    },
];

