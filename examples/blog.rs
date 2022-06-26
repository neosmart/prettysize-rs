#![allow(deprecated)]

// This is the code in the release post at
// https://neosmart.net/blog/2018/prettysize-for-rust/
extern crate size;

#[cfg(feature = "std")]
use size::{consts, Size};

#[cfg(feature = "std")]
fn main() {
    let bytes = 42 * consts::MiB;
    assert_eq!(bytes, 44040192);

    let bytes = Size::Mebibytes(42);
    assert_eq!(format!("{}", bytes), "42.0 MiB");
}

#[cfg(not(feature = "std"))]
fn main() {}
