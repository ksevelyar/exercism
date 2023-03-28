#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|num| u32_to_vlq(*num)).collect()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    println!("{:b}", 8192);
    let sum: u32 = bytes.iter().enumerate().rev().fold(0u32, |acc, (ind, byte)| {
        dbg!(byte, ind);
        println!("{:b}", byte);
        println!("{:b}", (byte & 0b1111111) >> (7 * ind));

        acc + acc | ((byte >> (7 * ind)) & 0b1111111) as u32
    });
    println!("{:b}", sum);

    Ok(vec![sum])
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
        .map(|(ind, byte)| match ind == max_ind {
            false => byte + 0b10000000,
            true => byte,
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
    assert_eq!(&[0x81, 0x80, 0x00], to_bytes(&[0x4000]).as_slice());

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
            0x00
        ],
        to_bytes(&[0x2000, 0x12_3456, 0x0fff_ffff, 0x00, 0x3fff, 0x4000]).as_slice()
    );
}

#[test]
fn from_bytes_specs() {
    // assert_eq!(Ok(vec![0x7f]), from_bytes(&[0x7f]));

    assert_eq!(Ok(vec![0x2000]), from_bytes(&[0xc0, 0x00]));
    //
    // assert_eq!(Ok(vec![0x1f_ffff]), from_bytes(&[0xff, 0xff, 0x7f]));
    //
    // assert_eq!(Ok(vec![0x20_0000]), from_bytes(&[0x81, 0x80, 0x80, 0x00]));
    //
    // assert_eq!(
    //     Ok(vec![0xffff_ffff]),
    //     from_bytes(&[0x8f, 0xff, 0xff, 0xff, 0x7f])
    // );
}
