pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    fn sequence(&self) -> impl Iterator<Item = u32> + Clone {
        (0..self.0).chain((1..(self.0 - 1)).rev()).cycle()
    }

    fn rows(&self) -> impl Iterator<Item = u32> {
        0..self.0
    }

    pub fn encode(&self, text: &str) -> String {
        let chars = text.chars().zip(self.sequence());

        self.rows()
            .flat_map(|rail| {
                chars
                    .clone()
                    .filter(move |(_ch, char_rail)| *char_rail == rail)
            })
            .map(|(ch, _)| ch)
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut positions: Vec<_> = self.sequence().take(cipher.len()).collect();
        positions.sort_unstable();
        let chars = &positions.iter().zip(cipher.chars());

        let mut rows_with_chars = self
            .rows()
            .map(|rail| {
                chars
                    .clone()
                    .filter(move |(char_rail, _ch)| **char_rail == rail)
                    .map(|(_, ch)| ch)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        rows_with_chars.iter_mut().for_each(|row| row.reverse());

        self.sequence()
            .take(cipher.len())
            .map(|row| (rows_with_chars[row as usize]).pop().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_encode_case(input: &str, rails: u32, expected: &str) {
        let rail_fence = RailFence::new(rails);

        assert_eq!(rail_fence.encode(input), expected);
    }

    fn process_decode_case(input: &str, rails: u32, expected: &str) {
        let rail_fence = RailFence::new(rails);

        assert_eq!(rail_fence.decode(input), expected);
    }

    #[test]
    fn test_encode_with_two_rails() {
        process_encode_case("XOXOXOXOXOXOXOXOXO", 2, "XXXXXXXXXOOOOOOOOO");
    }

    #[test]
    fn test_encode_with_three_rails() {
        process_encode_case("WEAREDISCOVEREDFLEEATONCE", 3, "WECRLTEERDSOEEFEAOCAIVDEN");
    }

    #[test]
    fn test_encode_with_ending_in_the_middle() {
        process_encode_case("EXERCISES", 4, "ESXIEECSR");
    }

    #[test]
    fn test_decode_with_three_rails() {
        process_decode_case("WECRLTEERDSOEEFEAOCAIVDEN", 3, "WEAREDISCOVEREDFLEEATONCE");
    }

    #[test]
    fn test_decode_with_five_rails() {
        process_decode_case("EIEXMSMESAORIWSCE", 5, "EXERCISMISAWESOME");
    }

    #[test]
    fn test_decode_with_six_rails() {
        process_decode_case(
            "133714114238148966225439541018335470986172518171757571896261",
            6,
            "112358132134558914423337761098715972584418167651094617711286",
        );
    }
}
