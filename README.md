# PrettySize, rust edition

A comprehensive file size crate for rust applications, meant to be light and effective.
Includes utilities for human-readable formatting of file sizes as well as converting
between different base-two and base-ten size units.

## Features

`PrettySize` provides

* definitions for the base-two and base-ten file size units defined as `pub const` in the
  `size` namespace, available both in abbreviated and unabridged forms (i.e. `size::EiB`
  and `size::EXBIBYTE` or `size::GB` and `size::GIGABYTE`),
* a `Unit` enum that defines the base-two and base-ten units,
* a `Size<T>` enum that can be used to hold a typed file size
  (e.g. `let size = Size::Terabytes(4);`),
* an `std::Display` impl for `Size` to display sizes in a human-readable format,
* a `Size.to_string(..)` method that allows you to specify the base of the human-readable
  units and their style (smart, abbreviated, or full and their lowercase variants)

## Usage

Cargo.toml:

```toml
[dependencies]
size = "0.1"
```

and in your code:

```rust
extern crate size;
use size::{Base, Size, Style};

fn main() {
        let byte_count = 42 * size::KiB;
        assert_eq!(43__008, byte_count);

        let byte_count = Size::Kilobytes(42);
        assert_eq!(42__000, byte_count.bytes());

        // `Size` can take any numeric type you throw at it
        let byte_count2 = Size::Mebibytes(0.040055);
        assert_eq!(byte_count.bytes(), byte_count2.bytes());

        // And for those of you that haven't yet drunk the base-two Kool-Aid:
        let byte_count = Size::Kilobytes(42);
        assert_eq!(byte_count.bytes(), 42_000);

        println!("{}, I say!", byte_count);
        // prints "41 KiB, I say!"

        println!("{}, I meant!", byte_count.to_string(Base::Base10, Style::Abbreviated));
        // prints "42 KB, I meant!"
}
```

## About

This project started off as a port of the
[PrettySize.NET](https://github.com/neosmart/PrettySize.net) library from C# to Rust. Like
the C# edition of this project. Rust's richer `enum` types and powerful generics made
implementing a custom `Size` generic over the number type without verbosity additionally
possible.

`PrettySize` is written and maintained by Mahmoud Al-Qudsi of NeoSmart Technologies and
released to the general public under the terms of the MIT public license.

## To-Do

* Providing a `FromStr` impl to parse file sizes,
* Implementing direct unit-to-unit conversion,
* Implementing format specifiers to allow using `format!(..)` directly to obtain output in
  the desired base and style,
* Bypassing double conversion when formatting `Size<T>` for integral `T` types,

Pull requests are welcomed!
