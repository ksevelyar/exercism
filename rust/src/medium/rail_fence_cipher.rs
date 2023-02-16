pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let chars = text
            .chars()
            .zip(((1..=self.0).chain((2..self.0).rev())).cycle());

        (1..=self.0)
            .flat_map(|rail| {
                chars
                    .clone()
                    .filter(move |(_ch, char_rail)| *char_rail == rail)
            })
            .map(|(ch, _)| ch)
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut positions: Vec<_> = ((1..=self.0).chain((2..self.0).rev()))
            .cycle()
            .take(cipher.chars().count())
            .collect();
        positions.sort_unstable();

        let rows = &positions.iter().zip(cipher.chars());

        let mut d = (1..=self.0)
            .map(|rail| {
                rows.clone()
                    .filter(move |(char_rail, ch)| **char_rail == rail)
                    .map(|(_, ch)| ch)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        ((0..self.0).chain((1..(self.0 - 1)).rev()))
            .cycle()
            .take(cipher.chars().count())
            .map(|row| {
                dbg!(row);
                d[row as usize].pop().unwrap()
            })
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .collect()
    }
}

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
}
