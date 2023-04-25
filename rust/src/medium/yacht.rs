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
pub fn score(_dice: Dice, _category: Category) -> u8 {
    unimplemented!("Solve the Yacht exercise");
}

#[test]
fn test_yacht() {
    let expected = 50;

    assert_eq!(score([5, 5, 5, 5, 5], Category::Yacht), expected);
}
