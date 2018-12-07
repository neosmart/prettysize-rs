use crate::Size;

#[test]
fn unit_tests() {
    assert_eq!("200 bytes", format!("{}", Size::Bytes(200)));
    assert_eq!("200 KiB", format!("{}", Size::Kibibytes(200)));
    assert_eq!("2.00 MiB", format!("{}", Size::Kibibytes(2048)));
}

#[test]
fn size_equality() {
    assert_eq!(
        Size::Bytes(200),
        Size::Bytes(200),
        "Testing equality of two identically-constructed sizes"
    );
    assert_eq!(
        Size::Mebibytes(2),
        Size::Kibibytes(2048),
        "Testing equality of two identical sizes expressed in different units"
    );
    assert_eq!(
        Size::Mebibytes(2u8),
        Size::Mebibytes(2f64),
        "Testing equality of two identical sizes expressed in different types"
    );
}
