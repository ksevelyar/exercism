#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    (0..4)
        .map(|ind| ((values[0] >> 7 * ind) & 0b1111111) as u8)
        .rev()
        .skip_while(|n| *n == 0)
        .enumerate()
        .map(|(ind, byte)| match ind == 0 {
            true => byte + 0b10000000,
            false => byte,
        })
        .collect()
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

#[test]
fn to_triple_byte() {
    assert_eq!(dbg!(&[0x81, 0x80, 0x00]), to_bytes(&[0x4000]).as_slice());

    assert_eq!(&[0xc0, 0x80, 0x00], to_bytes(&[0x10_0000]).as_slice());

    assert_eq!(&[0xff, 0xff, 0x7f], to_bytes(&[0x1f_ffff]).as_slice());
}
