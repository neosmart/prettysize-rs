# PrettySize, rust edition

[![crates.io](https://img.shields.io/crates/v/size.svg)](https://crates.io/crates/size) [![docs.rs](https://docs.rs/size/badge.svg)](https://docs.rs/size/latest/size/)

A comprehensive file size crate for rust applications, meant to be light and effective.
Includes utilities for human-readable formatting of file sizes as well as converting
between different base-two and base-ten size units and performing both mathematical and
logical operations on strongly-typed file sizes.

[See the crate documentation](https://docs.rs/size/latest/size/) for a more complete summary of
what this crate can do and how to use it.

## Features

`PrettySize` provides

* a `Size` type that can be used to hold a strongly-typed size
  (e.g. `let size = Size::from_gigabytes(4)`) and perform operations on it,
* definitions for the base-two and base-ten file size units defined as `pub const` in the
  `size::consts` namespace, available both in abbreviated and unabridged forms (i.e.
  `consts::KiB` and `consts::KIBIBYTE` or `consts::GB` and `consts::GIGABYTE`),
* an `std::Display` impl for `Size` to automatically display sizes in a human-readable
  format, automatically choosing the best size unit and numeric precision to
  give the nicest results (you can also use `Size::to_string()` instead).
* a `Size.format()` method that gives you more control over how sizes are converted
  to a textual representation, letting you to specify the base of the human-readable
  units and their style (smart, abbreviated, or full; plus their lowercase variants).
* mathematical and logical operations on strongly-typed `Size` values,
* full support for expressing negative sizes (e.g. the difference between two sizes, or the
  amount of space reclaimed on a disk)

This crate can also be used in `no_std` mode (by compiling with default features
disabled). This disables string conversion/formatting but keeps all the strongly-typed
size conversion and mathematical/logical operations available.

This crate is free of any dependencies.

## Usage

Cargo.toml:

```toml
[dependencies]
size = "0.4"
```

and in your code:

```rust
use size::{Base, Size};
// You can use/import consts representing base2/base10 sizes individually
// as (e.g.) size::KiB, or import all with `use size::consts::*`

fn main() {
  // Create strongly-typed sizes:
  let byte_count = Size::from_kilobytes(42);
  assert_eq!(42_000, byte_count.bytes());

  // Use predefined constants for the various units
  let byte_count = 42 * size::KiB;
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
```

## `no_std` usage

Add the crate to `Cargo.toml` with `default-features` disabled for `no_std` support:

```toml
[dependencies]
size = { version = "0.4", default-features = false }
```

Building in `no_std` mode disables support for floating point `Size` operations/conversions as well as string formatting and conversion.

## `serde` support

For serialization and deserialization support, add the `size` crate to your `Cargo.toml` with the `serde` feature enabled:

```toml
[dependencies]
size = { version = "0.4", features = [ "serde" ] }
```

**The `Size` type is serialized/deserialized transparently.** This means that it acts as if it were a `u64` field denoting the size in bytes. This was done to allow directly deserializing from network payloads from languages/apis that do not express sizes as strongly typed fields (and vice-versa).

As a concrete example, let's pretend you have the following struct that contains a `Size` field:

```rust
#[derive(Serialize, Deserialize)]
struct File {
    path: PathBuf,
    size: Size,
}
```

Using JSON as an example, the `File` type above will serialize to/from the following:

```json
{
    "path:" "/foo/bar",
    "size:" 1024
}
```

As you can see, the `size` field has been serialized directly to a numeric value (and not a `Size` object _containing_ that number value).

## About

This project started off as a port of Mahmoud's
[PrettySize.NET](https://github.com/neosmart/PrettySize.net) library from C# to Rust. Like
the C# edition of this project. Rust's richer `enum` types and powerful generics made
implementing a custom `Size` generic over the number type without verbosity additionally
possible. Its scope has since grown considerably.

# License

`PrettySize` is written and maintained by Mahmoud Al-Qudsi of NeoSmart Technologies and
released to the general public under the terms of the MIT public license.

## To-Do

* Providing a `FromStr` impl to parse file sizes ([coming
  soon!](https://github.com/neosmart/prettysize-rs/pull/3))

Pull requests are welcome!
