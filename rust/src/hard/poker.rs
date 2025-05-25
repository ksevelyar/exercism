/// Given a list of poker hands, return a list of those hands which win.
///
enum HandVariant {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

type SortedHand<'a> = Vec<(Vec<(u8, u8)>, &'a str)>;

const HAND_VARIANTS: [HandVariant; 9] = [
    HandVariant::StraightFlush,
    HandVariant::FourOfAKind,
    HandVariant::FullHouse,
    HandVariant::Flush,
    HandVariant::Straight,
    HandVariant::ThreeOfAKind,
    HandVariant::TwoPair,
    HandVariant::Pair,
    HandVariant::HighCard,
];

/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands = HAND_VARIANTS
        .iter()
        .find_map(|variant| get_variant_hands(variant, hands))
        .unwrap();

    let (max_score, _) = hands.iter().max_by_key(|(rank, _)| rank).unwrap();

    hands
        .iter()
        .filter(|(rank, _string)| rank == max_score)
        .map(|(_rank, string)| string)
        .copied()
        .collect()
}

fn get_variant_hands<'a>(variant: &HandVariant, hands: &[&'a str]) -> Option<SortedHand<'a>> {
    let parsed_hands = hands.iter().map(|&hand| (parse_hand(hand), hand));

    let filtered = match variant {
        HandVariant::StraightFlush => filter_straight_flush(parsed_hands),
        HandVariant::FourOfAKind => filter_by_frequency(parsed_hands, 4, 1),
        HandVariant::FullHouse => filter_full_house(parsed_hands),
        HandVariant::Flush => filter_flush(parsed_hands),
        HandVariant::Straight => filter_straight(parsed_hands),
        HandVariant::ThreeOfAKind => filter_by_frequency(parsed_hands, 3, 1),
        HandVariant::TwoPair => filter_by_frequency(parsed_hands, 2, 2),
        HandVariant::Pair => filter_by_frequency(parsed_hands, 2, 1),
        HandVariant::HighCard => parsed_hands.collect(),
    };

    if filtered.is_empty() {
        None
    } else {
        Some(filtered)
    }
}

fn filter_straight_flush<'a>(
    hands: impl Iterator<Item = (Vec<(u8, u8)>, &'a str)>,
) -> Vec<(Vec<(u8, u8)>, &'a str)> {
    hands
        .filter(|(parsed, raw)| is_straight(parsed) && is_flush(raw))
        .map(|(mut v, s)| {
            if v[0].0 == 14 {
                v[0].0 = 0;
            }
            (v, s)
        })
        .collect()
}

fn filter_straight<'a>(
    hands: impl Iterator<Item = (Vec<(u8, u8)>, &'a str)>,
) -> Vec<(Vec<(u8, u8)>, &'a str)> {
    hands
        .filter(|(parsed, _)| is_straight(parsed))
        .map(|(mut v, s)| {
            if v[0].0 == 14 {
                v[0].0 = 0;
            }
            (v, s)
        })
        .collect()
}

fn is_straight(parsed: &[(u8, u8)]) -> bool {
    parsed.iter().enumerate().all(|(i, &(rank, _))| {
        let min = parsed.iter().map(|(r, _)| *r).min().unwrap();
        (rank == min + 4 - i as u8) || (i == 0 && rank == 14)
    })
}

fn is_flush(raw_hand: &str) -> bool {
    let mut suits = raw_hand
        .split_whitespace()
        .map(|card| card.chars().last().unwrap());
    let first = suits.next().unwrap();
    suits.all(|suit| suit == first)
}

fn filter_flush<'a>(
    hands: impl Iterator<Item = (Vec<(u8, u8)>, &'a str)>,
) -> Vec<(Vec<(u8, u8)>, &'a str)> {
    hands.filter(|(_, raw)| is_flush(raw)).collect()
}

fn filter_by_frequency<'a>(
    hands: impl Iterator<Item = (Vec<(u8, u8)>, &'a str)>,
    freq: u8,
    count: usize,
) -> Vec<(Vec<(u8, u8)>, &'a str)> {
    hands
        .filter(|(parsed, _)| parsed.iter().filter(|&&(_, f)| f == freq).count() == count)
        .collect()
}

fn filter_full_house<'a>(
    hands: impl Iterator<Item = (Vec<(u8, u8)>, &'a str)>,
) -> Vec<(Vec<(u8, u8)>, &'a str)> {
    hands
        .filter(|(parsed, _)| {
            let has_three = parsed.iter().any(|&(_, f)| f == 3);
            let has_two = parsed.iter().any(|&(_, f)| f == 2);
            has_three && has_two
        })
        .collect()
}

fn card_rank(card: &str) -> u8 {
    let rank_str = if card.starts_with("10") {
        "10"
    } else {
        &card[..1]
    };

    match rank_str {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        n => n.parse().unwrap(),
    }
}

fn parse_hand(hand: &str) -> Vec<(u8, u8)> {
    let mut hand =
        hand.split_ascii_whitespace()
            .map(card_rank)
            .fold(Vec::new(), |mut acc, rank| {
                if let Some(pos) = acc.iter().position(|&(r, _)| r == rank) {
                    acc[pos].1 += 1;
                } else {
                    acc.push((rank, 1));
                }
                acc
            });

    hand.sort_unstable_by(|a, b| (b.1, b.0).cmp(&(a.1, a.0)));
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
    fn one_pair_beats_high_card() {
        let input = &["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2S 4H 6S 4D JH"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn highest_pair_wins() {
        let input = &["4S 2H 6S 2D JH", "2S 4H 6C 4D JD"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2S 4H 6C 4D JD"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_the_same_pair_high_card_wins() {
        let input = &["4H 4S AH JC 3D", "4C 4D AS 5D 6C"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4H 4S AH JC 3D"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn two_pairs_beats_one_pair() {
        let input = &["2S 8H 6S 8D JH", "4S 5H 4C 8C 5C"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4S 5H 4C 8C 5C"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_two_pairs_highest_ranked_pair_wins() {
        let input = &["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2S 8H 2D 8D 3H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_two_pairs_with_the_same_highest_ranked_pair_tie_goes_to_low_pair() {
        let input = &["2S QS 2C QD JH", "JD QH JS 8D QC"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["JD QH JS 8D QC"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_two_identically_ranked_pairs_tie_goes_to_remaining_card_kicker() {
        let input = &["JD QH JS 8D QC", "JS QS JC 2D QD"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["JD QH JS 8D QC"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_two_pairs_that_add_to_the_same_value_win_goes_to_highest_pair() {
        let input = &["6S 6H 3S 3H AS", "7H 7S 2H 2S AC"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["7H 7S 2H 2S AC"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn two_pairs_first_ranked_by_largest_pair() {
        let input = &["5C 2S 5S 4H 4C", "6S 2S 6H 7C 2C"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["6S 2S 6H 7C 2C"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn three_of_a_kind_beats_two_pair() {
        let input = &["2S 8H 2H 8D JH", "4S 5H 4C 8S 4H"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4S 5H 4C 8S 4H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_three_of_a_kind_tie_goes_to_highest_ranked_triplet() {
        let input = &["2S 2H 2C 8D JH", "4S AH AS 8C AD"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4S AH AS 8C AD"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn with_multiple_decks_two_players_can_have_same_three_of_a_kind_ties_go_to_highest_remaining_cards()
     {
        let input = &["5S AH AS 7C AD", "4S AH AS 8C AD"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4S AH AS 8C AD"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn a_straight_beats_three_of_a_kind() {
        let input = &["4S 5H 4C 8D 4H", "3S 4D 2S 6D 5C"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["3S 4D 2S 6D 5C"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn aces_can_end_a_straight_10_j_q_k_a() {
        let input = &["4S 5H 4C 8D 4H", "10D JH QS KD AC"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["10D JH QS KD AC"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn aces_can_start_a_straight_a_2_3_4_5() {
        let input = &["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4D AH 3S 2D 5C"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn aces_cannot_be_in_the_middle_of_a_straight_q_k_a_2_3() {
        let input = &["2C 3D 7H 5H 2S", "QS KH AC 2D 3S"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2C 3D 7H 5H 2S"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_with_a_straight_tie_goes_to_highest_ranked_card() {
        let input = &["4S 6C 7S 8D 5H", "5S 7H 8S 9D 6H"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["5S 7H 8S 9D 6H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn even_though_an_ace_is_usually_high_a_5_high_straight_is_the_lowest_scoring_straight() {
        let input = &["2H 3C 4D 5D 6H", "4S AH 3S 2D 5H"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2H 3C 4D 5D 6H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn flush_beats_a_straight() {
        let input = &["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2S 4S 5S 6S 7S"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_a_flush_tie_goes_to_high_card_down_to_the_last_one_if_necessary() {
        let input = &["2H 7H 8H 9H 6H", "3S 5S 6S 7S 8S"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2H 7H 8H 9H 6H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn full_house_beats_a_flush() {
        let input = &["3H 6H 7H 8H 5H", "4S 5H 4C 5D 4H"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4S 5H 4C 5D 4H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_a_full_house_tie_goes_to_highest_ranked_triplet() {
        let input = &["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["5H 5S 5D 8S 8D"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn with_multiple_decks_both_hands_have_a_full_house_with_the_same_triplet_tie_goes_to_the_pair()
    {
        let input = &["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["5H 5S 5D 9S 9D"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn four_of_a_kind_beats_a_full_house() {
        let input = &["4S 5H 4D 5D 4H", "3S 3H 2S 3D 3C"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["3S 3H 2S 3D 3C"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_four_of_a_kind_tie_goes_to_high_quad() {
        let input = &["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4S 5H 5S 5D 5C"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn with_multiple_decks_both_hands_with_identical_four_of_a_kind_tie_determined_by_kicker() {
        let input = &["3S 3H 2S 3D 3C", "3S 3H 4S 3D 3C"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["3S 3H 4S 3D 3C"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn straight_flush_beats_four_of_a_kind() {
        let input = &["4S 5H 5S 5D 5C", "7S 8S 9S 6S 10S"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["7S 8S 9S 6S 10S"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn aces_can_end_a_straight_flush_10_j_q_k_a() {
        let input = &["KC AH AS AD AC", "10C JC QC KC AC"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["10C JC QC KC AC"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn aces_can_start_a_straight_flush_a_2_3_4_5() {
        let input = &["KS AH AS AD AC", "4H AH 3H 2H 5H"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["4H AH 3H 2H 5H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn aces_cannot_be_in_the_middle_of_a_straight_flush_q_k_a_2_3() {
        let input = &["2C AC QC 10C KC", "QH KH AH 2H 3H"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2C AC QC 10C KC"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn both_hands_have_a_straight_flush_tie_goes_to_highest_ranked_card() {
        let input = &["4H 6H 7H 8H 5H", "5S 7S 8S 9S 6S"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["5S 7S 8S 9S 6S"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn even_though_an_ace_is_usually_high_a_5_high_straight_flush_is_the_lowest_scoring_straight_flush()
     {
        let input = &["2H 3H 4H 5H 6H", "4D AD 3D 2D 5D"];

        let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

        let expected = ["2H 3H 4H 5H 6H"].into_iter().collect::<HashSet<_>>();

        assert_eq!(output, expected);
    }
}
