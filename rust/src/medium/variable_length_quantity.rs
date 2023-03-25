#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|num| u32_to_vlq(*num)).collect()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {bytes:?} to a list of numbers")
}

fn u32_to_vlq(num: u32) -> Vec<u8> {
    if num == 0 {
        return vec![0];
    }

    let values = (0..=4)
        .map(|ind| ((num >> 7 * ind) & 0b1111111) as u8)
        .rev()
        .skip_while(|n| *n == 0);

    let max_ind = values.clone().count() - 1;

    values
        .enumerate()
        .map(|(ind, byte)| match ind != max_ind {
            true => byte + 0b10000000,
            false => byte,
        })
        .collect()
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

#[test]
fn to_bytes_multiple_values() {
    assert_eq!(&[0x40, 0x7f], to_bytes(&[0x40, 0x7f]).as_slice());

    assert_eq!(
        &[0x81, 0x80, 0x00, 0xc8, 0xe8, 0x56],
        to_bytes(&[0x4000, 0x12_3456]).as_slice()
    );

    assert_eq!(
        &[
            0xc0, 0x00, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x00, 0xff, 0x7f, 0x81, 0x80,
            0x00,
        ],
        to_bytes(&[0x2000, 0x12_3456, 0x0fff_ffff, 0x00, 0x3fff, 0x4000]).as_slice()
    );
}
