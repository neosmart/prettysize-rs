#![cfg(feature = "std")]

use crate::Size;

#[test]
fn unit_tests() {
    assert_eq!("200 bytes", format!("{}", Size::Bytes(200)));
    assert_eq!("200 KiB", format!("{}", Size::Kibibytes(200)));
    assert_eq!("2.00 MiB", format!("{}", Size::Kibibytes(2048)));
}

#[test]
fn negative_tests() {
    assert_eq!("-200 bytes", format!("{}", Size::Bytes(-200)));
    assert_eq!("-200 KiB", format!("{}", Size::Kibibytes(-200)));
    assert_eq!("-2.00 MiB", format!("{}", Size::Kibibytes(-2048)));
}

#[test]
fn integral_limits() {
    assert_eq!("8 EiB", format!("{}", Size::Bytes(i64::max_value())));
    assert_eq!("-8 EiB", format!("{}", Size::Bytes(i64::min_value())));

    assert_eq!("8 EiB", format!("{}", Size::Bytes(u64::max_value())));
    assert_eq!("0 bytes", format!("{}", Size::Bytes(u64::min_value())));
}

#[test]
fn size_equality() {
    assert_eq!(
        Size::Bytes(200),
        Size::Bytes(200),
        "Testing equality of two identically-constructed sizes"
    );
    assert_eq!(
        Size::Mebibytes(2),
        Size::Kibibytes(2048),
        "Testing equality of two identical sizes expressed in different units"
    );
    assert_eq!(
        Size::Mebibytes(2u8),
        Size::Mebibytes(2f64),
        "Testing equality of two identical sizes expressed in different types"
    );
    assert_eq!(
        Size::Kibibytes(2u8),
        Size::Kilobytes(2.048),
        "Testing equality of two identical sizes expressed in different types"
    );
    assert_eq!(
        &Size::Kibibytes(2u8),
        &Size::Kilobytes(2.048),
        "Testing equality of two Size references"
    );
}

#[test]
fn size_cmp() {
    assert!(Size::Bytes(1) > Size::Bytes(0), "Comparison of two Size types directly");
    assert!(&Size::KiB(1) >= &Size::KB(1), "Comparison of two Size types via their references");
}

#[test]
fn size_addition() {
    // as a reference...
    let size = &Size::Mebibytes(20) + &Size::Mebibytes(22);
    assert_eq!(size, Size::Mebibytes(42));

    // and not as a reference
    let size = Size::Mebibytes(20) + Size::Mebibytes(22_f64);
    assert_eq!(size, Size::Mebibytes(42));
}

#[test]
fn primitive_multiplication() {
    let size = &Size::Gigabytes(12) * 7;
    assert_eq!(size.bytes(), 84000000000);
    let size = Size::Gigabytes(12) * 7;
    assert_eq!(size.bytes(), 84000000000);

    // and the other way around
    let size = 7 * Size::Gigabytes(12);
    assert_eq!(size.bytes(), 84000000000);

    // and with other types
    let size = &Size::Gigabytes(12) * 7.0;
    assert_eq!(size.bytes(), 84000000000);
    let size = 7.0 * &Size::Gigabytes(12);
    assert_eq!(size.bytes(), 84000000000);
}

#[test]
fn primitive_division() {
    let size = &Size::Gigabytes(12) / 13f64;
    assert_eq!(size.bytes(), 923076923);

    let size = Size::Gigabytes(12.0) / 13;
    assert_eq!(size.bytes(), 923076923);
}

/// Floats that cannot be expressed as an `i64` may be instantiated, but give undefined results
/// when operated on.
#[test]
fn nan_size() {
    let size = Size::Bytes(f32::NAN);
    let _ = size + Size::Bytes(1);
    let _ = format!("{}", size);
}

/// Floats that cannot be expressed as an `i64` may be instantiated, but give undefined results
/// when operated on. The code below panics in debug mode but continues with undefined results in
/// release mode.
#[test]
fn overflow_size() {
    use std::panic;

    // This value is well out of the range of an i64, but is a perfectly valid floating point value.
    let result = panic::catch_unwind(|| {
        let _ = Size::Bytes(7.3E200_f64) + Size::KiB(2);
    });

    if cfg!(debug_assertions) {
        assert!(result.is_err());
    } else {
        assert!(result.is_ok());
    }
}
