#![cfg(feature = "std")]
#![allow(deprecated)]

use crate::Size;

#[test]
fn unit_tests() {
    assert_eq!("200 bytes", format!("{}", Size::from_bytes(200)));
    assert_eq!("200 KiB", format!("{}", Size::from_kibibytes(200)));
    assert_eq!("2.00 MiB", format!("{}", Size::from_kibibytes(2048)));
}

#[test]
fn negative_tests() {
    assert_eq!("-200 bytes", format!("{}", Size::from_bytes(-200)));
    assert_eq!("-200 KiB", format!("{}", Size::from_kibibytes(-200)));
    assert_eq!("-2.00 MiB", format!("{}", Size::from_kibibytes(-2048)));
}

#[test]
fn integral_limits() {
    assert_eq!("8 EiB", format!("{}", Size::from_bytes(i64::max_value())));
    assert_eq!("-8 EiB", format!("{}", Size::from_bytes(i64::min_value())));

    assert_eq!("8 EiB", format!("{}", Size::from_kib(u64::max_value())));
    assert_eq!("0 bytes", format!("{}", Size::from_kib(u64::min_value())));

    // Also test for the old-style API, which does no math at the point of creation
    assert_eq!("8 EiB", format!("{}", Size::Bytes(u64::max_value())));
    assert_eq!("0 bytes", format!("{}", Size::Bytes(u64::min_value())));
}

#[test]
fn float_limits() {
    assert_eq!("8 EiB", format!("{}", Size::from_kib(f64::MAX)));
    assert_eq!("-8 EiB", format!("{}", Size::from_kib(f64::MIN)));

    // Also test for the old-style API, which does no math at the point of creation
    assert_eq!("8 EiB", format!("{}", Size::Bytes(f64::MAX)));
    assert_eq!("-8 EiB", format!("{}", Size::Bytes(f64::MIN)));
}

#[test]
/// Make sure invalid floats don't panic. The *actual* result is officially undefined by this
/// crate's API contract.
fn invalid_floats() {
    assert_eq!("0 bytes", format!("{}", Size::from_kib(f64::NAN)));
    assert_eq!("8 EiB", format!("{}", Size::from_kib(f64::INFINITY)));
    assert_eq!("-8 EiB", format!("{}", Size::from_kib(f64::NEG_INFINITY)));

    // Also test for the old-style API, which does no math at the point of creation
    assert_eq!("0 bytes", format!("{}", Size::Bytes(f64::NAN)));
    assert_eq!("8 EiB", format!("{}", Size::Bytes(f64::INFINITY)));
    assert_eq!("-8 EiB", format!("{}", Size::Bytes(f64::NEG_INFINITY)));
}

#[test]
fn size_equality() {
    assert_eq!(
        Size::from_bytes(200),
        Size::from_bytes(200),
        "Testing equality of two identically-constructed sizes"
    );
    assert_eq!(
        Size::from_mib(2),
        Size::from_kib(2048),
        "Testing equality of two identical sizes expressed in different units"
    );
    assert_eq!(
        Size::from_mib(2u8),
        Size::from_mib(2f64),
        "Testing equality of two identical sizes expressed in different types"
    );
    assert_eq!(
        Size::from_mib(2u8),
        Size::from_kib(2048),
        "Testing equality of two identical sizes expressed in different types"
    );
    assert_eq!(
        &Size::from_bytes(2097),
        &Size::from_kib(2.048),
        "Testing equality of two Size references"
    );
}

#[test]
fn size_cmp() {
    // Use legacy/backwards-compatible syntax:
    assert!(Size::Bytes(1) > Size::Bytes(0), "Comparison of two Size types directly");
    assert!(
        &Size::KiB(1) >= &Size::KB(1),
        "Comparison of two Size types via their references"
    );
}

#[test]
fn size_addition() {
    // as a reference...
    let size = &Size::from_mib(20) + &Size::from_mib(22);
    assert_eq!(size, Size::Mebibytes(42));

    // and not as a reference
    let size = Size::from_mib(20) + Size::from_mib(22_f64);
    assert_eq!(size, Size::Mebibytes(42));
}

#[test]
fn primitive_multiplication() {
    let size = &Size::from_gb(12) * 7;
    assert_eq!(size.bytes(), 84000000000);
    let size = Size::from_gb(12) * 7;
    assert_eq!(size.bytes(), 84000000000);

    // and the other way around
    let size = 7 * Size::from_gb(12);
    assert_eq!(size.bytes(), 84000000000);

    // and with other types
    let size = &Size::from_gb(12) * 7.0;
    assert_eq!(size.bytes(), 84000000000);
    let size = 7.0 * &Size::from_gb(12);
    assert_eq!(size.bytes(), 84000000000);
}

#[test]
fn primitive_division() {
    let size = &Size::from_gb(12) / 13f64;
    assert_eq!(size.bytes(), 923076923);

    let size = Size::from_gb(12.0) / 13;
    assert_eq!(size.bytes(), 923076923);
}

/// Floats that cannot be expressed as an `i64` may be instantiated, but give undefined results
/// when operated on.
#[test]
fn nan_size() {
    let size = Size::from_kib(f32::NAN);
    let _ = size + Size::from_bytes(1);
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
        let _ = Size::from_kb(7.3E200_f64) + Size::from_kib(2);
    });

    if cfg!(debug_assertions) {
        assert!(result.is_err());
    } else {
        assert!(result.is_ok());
    }
}
