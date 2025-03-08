use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone, Debug)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    shift: usize,
}

impl<'a> Xorcism<'a> {
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref(),
            shift: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let mut key_iter = self.key.iter().cycle().skip(self.shift);
        self.shift = (self.shift + data.len()) % self.key.len();

        for data_byte in data.iter_mut() {
            let key_byte = key_iter.next().unwrap();

            *data_byte ^= *key_byte
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator,
        Data::Item: Borrow<u8>,
    {
        let key_iter = self.key.iter().cycle().skip(self.shift);

        data.into_iter()
            .map(|data_byte| *data_byte.borrow())
            .zip(key_iter)
            .map(|(data_byte, key_byte)| {
                self.shift = (self.shift + 1) % self.key.len();

                data_byte ^ key_byte
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "abcde";
    const INPUT: &str = "123455";
    const EXPECT: &[u8] = &[80, 80, 80, 80, 80, 84];

    #[test]
    fn munge_in_place_identity() {
        let mut xs = Xorcism::new(&[0]);

        let input = "This is super-secret, cutting edge encryption, folks.".as_bytes();

        let mut output = input.to_owned();

        xs.munge_in_place(&mut output);

        assert_eq!(&input, &output);
    }

    #[test]
    fn munge_in_place_roundtrip() {
        let mut xs1 = Xorcism::new(&[1, 2, 3, 4, 5]);

        let mut xs2 = Xorcism::new(&[1, 2, 3, 4, 5]);

        let input = "This is super-secret, cutting edge encryption, folks.".as_bytes();

        let mut cipher = input.to_owned();

        xs1.munge_in_place(&mut cipher);

        assert_ne!(&input, &cipher);

        let mut output = cipher;

        xs2.munge_in_place(&mut output);

        assert_eq!(&input, &output);
    }

    #[test]
    fn munge_in_place_stateful() {
        let mut xs = Xorcism::new(&[1, 2, 3, 4, 5]);

        let input = "This is super-secret, cutting edge encryption, folks.".as_bytes();

        let mut cipher1 = input.to_owned();

        let mut cipher2 = input.to_owned();

        xs.munge_in_place(&mut cipher1);

        xs.munge_in_place(&mut cipher2);

        assert_ne!(&input, &cipher1);

        assert_ne!(&input, &cipher2);

        assert_ne!(&cipher1, &cipher2);
    }

    #[test]
    fn statefulness() {
        // we expect Xorcism to be stateful: at the end of a munging run, the key has rotated.
        // this means that until the key has completely rotated around, equal inputs will produce
        // unequal outputs.

        let key = &[0, 1, 2, 3, 4, 5, 6, 7];

        let input = &[0b1010_1010, 0b0101_0101];

        let mut xs = Xorcism::new(&key);

        let out1: Vec<_> = xs.munge(input).collect();

        let out2: Vec<_> = xs.munge(input).collect();

        let out3: Vec<_> = xs.munge(input).collect();

        let out4: Vec<_> = xs.munge(input).collect();

        let out5: Vec<_> = xs.munge(input).collect();

        assert_ne!(out1, out2);

        assert_ne!(out2, out3);

        assert_ne!(out3, out4);

        assert_ne!(out4, out5);

        assert_eq!(out1, out5);
    }

    #[test]
    fn munge_in_place_slice() {
        // we transform the input into a `Vec<u8>` despite its presence in this
        // module because of the more restricted syntax that this function accepts

        let mut input = INPUT.as_bytes().to_vec();

        let original = input.clone();

        // in-place munging is stateful on Xorcism, so clone it
        // to ensure the keys positions stay synchronized
        let mut xorcism1 = Xorcism::new(KEY);

        let mut xorcism2 = xorcism1.clone();

        xorcism1.munge_in_place(&mut input);

        assert_eq!(input.len(), original.len());

        assert_ne!(input, original);

        assert_eq!(input, EXPECT);

        xorcism2.munge_in_place(&mut input);

        assert_eq!(input, original);
    }

    #[test]
    fn munges() {
        let owned_input = INPUT.as_bytes().to_vec();

        let mut xorcism = Xorcism::new(KEY);

        let result: Vec<u8> = xorcism.munge(owned_input).collect();

        assert_eq!(INPUT.len(), result.len());

        assert_ne!(INPUT.as_bytes(), result);

        assert_eq!(result, EXPECT);
    }
}
