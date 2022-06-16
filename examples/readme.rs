extern crate size;
#[cfg(feature = "std")]
use size::{Base, Size, Style};

#[cfg(feature = "std")]
fn main() {
    let byte_count = 42 * size::KiB;
    assert_eq!(43__008, byte_count);

    let byte_count = Size::Kilobytes(42);
    assert_eq!(42__000, byte_count.bytes());

    // `Size` can take any numeric type you throw at it
    let byte_count2 = Size::Mebibytes(0.040055);
    assert_eq!(byte_count.bytes(), byte_count2.bytes());

    // And for those of you that haven't yet drank the base-two Kool-Aid:
    let byte_count = Size::Kilobytes(42);
    assert_eq!(byte_count.bytes(), 42_000);

    println!("{}, I say!", byte_count);
    // prints "41 KiB, I say!"

    println!(
        "{}, I meant!",
        byte_count.to_string(Base::Base10, Style::Abbreviated)
    );
    // prints "42 KB, I meant!"
}

#[cfg(not(feature = "std"))]
fn main() {}
