use std::collections::HashMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let max_score = hands
        .iter()
        .map(|hand| find_score(hand))
        .max()
        .unwrap_or_default();

    hands
        .iter()
        .filter(|hand| find_score(hand) == max_score)
        .copied()
        .collect()
}

fn find_score(hand: &str) -> Vec<u8> {
    let mut hand: Vec<u8> = hand
        .split_ascii_whitespace()
        .map(|card| {
            let rank = card.as_bytes()[0];
            rank
        })
        .collect();

    let counts: HashMap<_, _> =
        hand.iter()
            .copied()
            .fold(HashMap::with_capacity(5), |mut acc, card| {
                *acc.entry(card).or_insert(0) += 1;
                acc
            });

    hand.sort_by(|a, b| (counts.get(b).unwrap(), b).cmp(&(counts.get(a).unwrap(), a)));

    hand
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn single_hand_always_wins() {
        let input = &["4S 5S 7H 8D JC"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4S 5S 7H 8D JC"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn highest_card_out_of_all_hands_wins() {
        let input = &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["3S 4S 5D 6H JH"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn a_tie_has_multiple_winners() {
        let input = &[
            "4D 5S 6S 8D 3C",
            "2S 4C 7S 9H 10H",
            "3S 4S 5D 6H JH",
            "3H 4H 5C 6C JD",
        ];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"]
            .into_iter()
            .collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_hands_with_the_same_high_cards_tie_compares_next_highest_ranked_down_to_last_card()
    {
        let input = &["3S 5H 6S 8D 7H", "2S 5D 6D 8C 7S"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["3S 5H 6S 8D 7H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn winning_high_card_hand_also_has_the_lowest_card() {
        let input = &["2S 5H 6S 8D 7H", "3S 4D 6D 8C 7S"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2S 5H 6S 8D 7H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn one_pair_beats_high_card() {
        let input = &["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2S 4H 6S 4D JH"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }
}
