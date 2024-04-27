use serde::{Deserialize, Serialize};
use size::consts::*; // Import consts like KB, MiB, etc.
use size::Size; // Core type for all size operations

fn main() {
    // Flexible construction options
    let s = Size::from_bytes(440 * KB) + Size::from_mib(12.9);
    println!("The pretty file size {s}"); // 13.3 MiB

    // Mathematical operations on sizes and scalar values
    let double = Size::from_kb(0.668) * 2 + Size::from_bytes(1);
    assert_eq!(double.bytes(), 1337);

    // Parse sizes from strings in almost any format
    let parsed = Size::from_str("43.008 KB").unwrap();
    assert_eq!(Size::from_kib(42.0), parsed);

    #[derive(Debug, Deserialize, Serialize)]
    struct File {
        name: String,
        size: Size,
        disk_size: Size,
    }

    // Serialize and and deserialize from byte values or strings
    let _: File = serde_json::from_str(
        r#"{
            "name": "Hello.txt",
            "size": "12.92 gigabytes",
            "disk_size": 12920000000
        }"#,
    )
    .unwrap();
}
