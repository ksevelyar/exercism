#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    println!("{}", values[0]);
    println!("{:032b}", values[0]);

    let o1 = values[0] & 0b1111111;
    let o2 = (values[0] >> 7) & 0b1111111;
    let o3 = (values[0] >> 14) & 0b1111111;

    let bytes: Vec<u8> = (0..4)
        .map(|ind| {
            let byte = ((values[0] >> 7 * ind) & 0b1111111) as u8;
            // match ind == 0 {
            //     true => byte,
            //     false => byte
            // }
            byte
        })
        .take_while(|b| *b != 0)
        .collect();
    dbg!(&bytes);

    bytes.iter().for_each(|byte| println!("{:07b}", byte));

    println!("\n\ncheck");
    println!("{:07b}", o1);
    println!("{:07b}", o2);
    println!("{:07b}", o3);

    bytes
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

    // assert_eq!(&[0xc0, 0x00], to_bytes(&[0x2000]).as_slice());
    //
    // assert_eq!(&[0xff, 0x7f], to_bytes(&[0x3fff]).as_slice());
}
