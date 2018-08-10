use Size;

#[test]
fn unit_tests() {
    assert_eq!("200 bytes", format!("{}", Size::Bytes(200)));
    assert_eq!("200 KiB", format!("{}", Size::Kibibytes(200)));
    assert_eq!("2.00 MiB", format!("{}", Size::Kibibytes(2048)));
}
