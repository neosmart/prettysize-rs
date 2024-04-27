use std::error::Error;
use std::str::FromStr;

use crate::consts::*;
use crate::Size;

/// Represents an error parsing a `Size` from a string representation.
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ParseSizeError;

impl Error for ParseSizeError {}
impl core::fmt::Display for ParseSizeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Error parsing Size")
    }
}

impl Size {
    /// Parse a string representation of size to a `Size` value.
    ///
    /// Supports any mix-and-match of the following text formats:
    /// * 1234
    /// * 1234 b/kb/mb/etc
    /// * 1234 B/KB/MB/etc
    /// * 1234 B/KiB/MiB/etc
    /// * 1234MB
    /// * 12.34 GB
    /// * 1234 byte/kilobyte/terabyte/etc
    /// * 1234 bytes/kilobytes/terabytes/etc
    /// * 12.34 Kibibytes/MegaBytes/etc
    ///
    /// # Example
    ///
    /// ```rust
    /// use size::Size;
    ///
    /// let size = Size::from_str("12.34 KB").unwrap();
    /// assert_eq!(size.bytes(), 12_340);
    /// ```
    pub fn from_str(s: &str) -> Result<Size, crate::ParseSizeError> {
        FromStr::from_str(s)
    }
}

/// This test just ensures everything is wired up correctly between the member function
/// `[Size::from_str()]` and the `FromStr` trait impl.
#[test]
fn from_str() {
    let input = "12.34 kIloByte";
    let parsed = Size::from_str(input);
    let expected = Size::from_bytes(12.34 * crate::consts::KB as f64);
    assert_eq!(parsed, Ok(expected));
}

#[test]
fn parse() {
    let size = "12.34 kIloByte".parse();
    assert_eq!(size, Ok(Size::from_bytes(12 * KB + 340)));
}

impl FromStr for Size {
    type Err = ParseSizeError;

    fn from_str(s: &str) -> Result<Size, Self::Err> {
        let s = s.trim();

        // Try to split before the first unit char in the input. This supports the (unadvertised)
        // ability to parse scientific notation w/o spaces between scalar and unit.
        let (num_str, unit) = match s.rfind(|c: char| !c.is_ascii_alphabetic()).map(|i| i + 1) {
            None => (s, ""), // just a number, no unit
            Some(idx) => s.split_at(idx),
        };

        let number: f64 = num_str.trim_end().parse().map_err(|_| ParseSizeError)?;
        let unit = unit.to_lowercase();

        let multiplier = match unit.as_str().trim_end_matches('s') {
            "" | "b" | "byte" => B,
            "kb" | "kilobyte" => KB,
            "mb" | "megabyte" => MB,
            "gb" | "gigabyte" => GB,
            "tb" | "terabyte" => TB,
            "pb" | "petabyte" => PB,
            "eb" | "exabyte" => EB,

            "kib" | "kibibyte" => KiB,
            "mib" | "mebibyte" => MiB,
            "gib" | "gibibyte" => GiB,
            "tib" | "tebibyte" => TiB,
            "pib" | "pebibyte" => PiB,
            "eib" | "exbibyte" => EiB,

            _ => return Err(ParseSizeError),
        };

        Ok(Size::from_bytes(number * multiplier as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bare_bytes() {
        assert_eq!(Size::from_str("1234"), Ok(Size { bytes: 1234 }));
        assert_eq!(Size::from_str(" 1234 "), Ok(Size { bytes: 1234 })); // Leading and trailing whitespace
    }

    #[test]
    fn parse_abbr_unit() {
        let tests = vec![
            ("1234B", 1234),
            ("1234 KB", 1234 * KB),
            ("1234KiB", 1234 * KiB),
            ("12.34 MB", (12.34 * MB as f64) as i64),
            ("12.34MiB", (12.34 * MiB as f64) as i64),
            (" 1234 GB ", 1234 * GB),
        ];

        for (input, expected) in tests {
            assert_eq!(Size::from_str(input), Ok(Size { bytes: expected }));
        }
    }

    #[test]
    fn parse_full_unit() {
        let tests = vec![
            ("1234 bytes", 1234),
            ("1234 kilobytes", 1234 * KB),
            ("1234 kibibytes", 1234 * KiB),
            ("12.34 gigabytes", (12.34 * GB as f64) as i64),
            ("12.34   gibibytes", (12.34 * GiB as f64) as i64),
        ];

        for (input, expected) in tests {
            assert_eq!(Size::from_str(input), Ok(Size { bytes: expected }));
        }
    }

    #[test]
    fn parse_invalid_inputs() {
        let tests = vec![
            "Not a number",
            "1234 XB",   // Unknown suffix
            "12..34 MB", // Invalid number format
        ];

        for input in tests {
            assert_eq!(dbg!(Size::from_str(input)), Err(ParseSizeError));
        }
    }

    #[test]
    fn parse_boundary() {
        assert_eq!(Size::from_str("42.0"), Ok(Size::from_bytes(42)));
        assert_eq!(Size::from_str("42.0kib "), Ok(Size::from_bytes(42 * KiB)));
    }

    #[test]
    fn parse_scientific() {
        assert_eq!(Size::from_str("0.423E3"), Ok(Size::from_bytes(423)));
        assert_eq!(Size::from_str("423E-3 mb"), Ok(Size::from_bytes(423_000)));
        assert_eq!(Size::from_str("0.423e3kb"), Ok(Size::from_bytes(423_000)));
    }
}
