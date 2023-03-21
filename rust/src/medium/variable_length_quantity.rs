#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    println!("{}", values[0]);
    println!("{:b}", values[0]);

    let bytes: [u8; 4] = unsafe { std::mem::transmute(values[0]) };
    dbg!(bytes);

    unimplemented!("Convert the values {values:?} to a list of bytes")
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {bytes:?} to a list of numbers")
}

#[test]
fn to_single_byte() {
    assert_eq!(&[0x7f], to_bytes(&[0x7f]).as_slice());
}

#[test]
fn to_double_byte() {
    assert_eq!(&[0x81, 0x00], to_bytes(&[0x80]).as_slice());

    assert_eq!(&[0xc0, 0x00], to_bytes(&[0x2000]).as_slice());

    assert_eq!(&[0xff, 0x7f], to_bytes(&[0x3fff]).as_slice());
}
