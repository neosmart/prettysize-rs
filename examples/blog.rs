// This is the code in the release post at
// https://neosmart.net/blog/2018/prettysize-for-rust/
extern crate size;

use size::{MiB, Size};

fn main() {
    let bytes = 42 * MiB;
    assert_eq!(bytes, 44040192);

    let bytes = Size::Mebibytes(42);
    assert_eq!(format!("{}", bytes), "42.0 MiB");
}
