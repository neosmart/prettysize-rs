#![allow(deprecated)]

use crate::Size;

#[test]
fn nostd_add() {
    let s1 = Size::from_kib(12);
    let s2 = Size::from_kib(24);
    let sum = s1 + s2;
    assert_eq!(sum.bytes(), Size::KiB(36).bytes());
}

#[test]
fn nostd_sub() {
    let s1 = Size::from_kib(24_i32);
    let s2 = Size::from_kib(12_i64);
    let sum = s1 - s2;
    assert_eq!(sum.bytes(), Size::KiB(12).bytes());
}

#[test]
fn nostd_neg_sub() {
    let s1 = Size::from_kib(12_u64);
    let s2 = Size::from_kib(24_i64);
    let sum = s1 - s2;
    assert_eq!(sum.bytes(), Size::from_kib(-12).bytes());
}

#[test]
fn nostd_bytes() {
    let s1 = Size::from_kib(36);
    let s2 = Size::from_bytes(36 << 10);
    assert_eq!(s1.bytes(), s2.bytes());
    assert_eq!(s1.bytes(), 36 << 10);
}

#[test]
fn nostd_integral_limits() {
    // Test the old-style API, which does no math at the point of creation
    assert_eq!(Size::from_bytes(i64::MAX), Size::Bytes(u64::MAX));
    assert_eq!(Size::from_bytes(0), Size::Bytes(u64::MIN));
    assert_eq!(Size::from_bytes(i64::MAX), Size::Bytes(u64::MAX - 1));
}

#[test]
fn nostd_add_assign() {
    let mut size = Size::from_mib(20);
    size += Size::from_mib(22);
    assert_eq!(size, Size::Mebibytes(42));
}

#[test]
fn nostd_mul_assign() {
    let mut size = Size::from_gb(12);
    size *= 7;
    assert_eq!(size.bytes(), 84000000000);
}

#[test]
fn nostd_sub_assign() {
    let mut s1 = Size::from_kib(24_i32);
    let s2 = Size::from_kib(12_i64);
    s1 -= s2;
    assert_eq!(s1.bytes(), Size::KiB(12).bytes());
}

#[test]
fn nostd_neg_sub_assign() {
    let mut s1 = Size::from_kib(12_u64);
    let s2 = Size::from_kib(24_i64);
    s1 -= s2;
    assert_eq!(s1.bytes(), Size::from_kib(-12).bytes());
}
