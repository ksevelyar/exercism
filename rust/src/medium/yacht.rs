pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(dice: Dice, category: Category) -> u8 {
    use Category::*;

    match category {
        Ones => count(1, dice),
        Twos => count(2, dice),
        Threes => count(3, dice),
        Fours => count(4, dice),
        Fives => count(5, dice),
        Sixes => count(6, dice),
        FullHouse => {
            let dices = (1..=6)
                .map(|x| (dice.iter().filter(|i| **i == x)).count())
                .filter(|x| *x == 2 || *x == 3)
                .sum();

            match dices {
                5 => dice.iter().sum(),
                _ => 0,
            }
        }
        FourOfAKind => {
            let kind = (1..=6).find(|x| (dice.iter().filter(|i| **i == *x)).count() >= 4);
            match kind {
                Some(kind) => dice.iter().filter(|i| **i == kind).take(4).sum(),
                None => 0,
            }
        }
        LittleStraight => {
            let mut dice = dice.to_vec();
            dice.sort_unstable();
            dice.dedup();

            match &dice[..] {
                [1, 2, 3, 4, 5] => 30,
                _ => 0,
            }
        }
        BigStraight => {
            let mut dice = dice.to_vec();
            dice.sort_unstable();
            dice.dedup();

            match &dice[..] {
                [2, 3, 4, 5, 6] => 30,
                _ => 0,
            }
        }
        Choice => dice.iter().sum(),
        Yacht => match dice.iter().all(|item| *item == dice[0]) {
            true => 50,
            false => 0,
        },
    }
}

fn count(number: u8, dice: Dice) -> u8 {
    number * dice.iter().filter(|x| **x == number).count() as u8
}

#[test]
fn test_yacht() {
    let expected = 50;

    assert_eq!(score([5, 5, 5, 5, 5], Category::Yacht), expected);
}

#[test]
fn test_four_of_a_kind() {
    let expected = 24;

    assert_eq!(score([6, 6, 4, 6, 6], Category::FourOfAKind), expected);
}

#[test]
fn test_four_of_a_kind_is_not_a_full_house() {
    let expected = 0;

    assert_eq!(score([1, 4, 4, 4, 4], Category::FullHouse), expected);
}
