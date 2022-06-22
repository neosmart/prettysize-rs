#![allow(deprecated)]

#[cfg(feature = "std")]
mod std {
    use size::{Base, Size, Style};
    use size::consts;

    pub(super) fn main() {
      // Create strongly-typed sizes:
      let byte_count = Size::Kilobytes(42);
      assert_eq!(42_000, byte_count.bytes());

      // Use predefined constants for the various units
      let byte_count = 42 * consts::KiB;
      assert_eq!(43_008, byte_count);

      // `Size` can take any numeric type you throw at it
      let byte_count = Size::MiB(0.040055);
      assert_eq!(byte_count.bytes(), 42_000);

      // And for those of you that haven't yet drunk the base-two Kool-Aid:
      let byte_count = Size::Kilobytes(42);
      assert_eq!(byte_count.bytes(), 42_000);

      println!("{}, I say!", byte_count);
      // prints "41 KiB, I say!"

      // Override the default choice of base-2 units
      println!("{}, I meant!", byte_count.to_string(Base::Base10, Style::Abbreviated));
      // prints "42 KB, I meant!"

      // Add and subtract strongly-typed sizes, even with different underlying types
      let sum = Size::MB(1.0) + Size::KB(200);
      assert_eq!(sum.bytes(), 1_200_000);

      // Multiply and divide strongly-typed sizes by scalar values
      let new_size = Size::MiB(2) * 2;
      assert_eq!(new_size, Size::MiB(4));

      // Compare sizes for equality or order
      let size1 = Size::Gigabytes(2);
      let size2 = Size::GiB(1.99);
      assert!(size1 < size2);
    }
}

#[cfg(not(feature = "std"))]
fn main() {}
#[cfg(feature = "std")]
fn main() { std::main() }
