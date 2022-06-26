#[cfg(feature = "std")]
mod std {
    use size::consts;
    use size::{Base, Size};

    pub(super) fn main() {
        // Create strongly-typed sizes:
        let byte_count = Size::from_kilobytes(42);
        assert_eq!(42_000, byte_count.bytes());

        // Use predefined constants for the various units
        let byte_count = 42 * consts::KiB;
        assert_eq!(43_008, byte_count);

        // `Size` can take any numeric type you throw at it
        let byte_count = Size::from_mib(0.040055);
        assert_eq!(byte_count.bytes(), 42_000);

        // And for those of you that haven't yet drunk the base-two Kool-Aid:
        let file_size = Size::from_kb(42);
        assert_eq!(file_size.bytes(), 42_000);

        println!("{}, I say!", file_size);
        // prints "41 KiB, I say!"

        // Override the default choice of base-2 units
        println!("{}, I meant!", file_size.format().with_base(Base::Base10));
        // prints "42 KB, I meant!"

        // Add and subtract strongly-typed sizes, even with different underlying types
        let sum = Size::from_mb(1.0) + Size::from_kb(200);
        assert_eq!(sum.bytes(), 1_200_000);

        // Multiply and divide strongly-typed sizes by scalar values
        let new_size = Size::from_mib(2) * 2;
        assert_eq!(new_size, Size::from_mib(4));

        // Compare sizes for equality or order
        let size1 = Size::from_gigabytes(2);
        let size2 = Size::from_gibibytes(1.99);
        assert!(size1 < size2);
    }
}

#[cfg(not(feature = "std"))]
fn main() {}
#[cfg(feature = "std")]
fn main() {
    std::main()
}
