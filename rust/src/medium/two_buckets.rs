#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

// NOTE opposite starting point disallowed
fn pour_from_big_to_small() -> Option<BucketStats> {
    let moves = 0;
    let other_bucket = 0;

    Some(BucketStats {
        moves,
        goal_bucket: Bucket::Two,
        other_bucket,
    })
}

fn pour_from_small_to_big(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    b1: u8,
    b2: u8,
    moves: u8,
) -> Option<BucketStats> {
    if b1 == goal {
        return Some(BucketStats {
            moves,
            goal_bucket: Bucket::One,
            other_bucket: b2,
        });
    }

    match (b1, b2) {
        (0, b) => pour_from_small_to_big(capacity_1, capacity_2, goal, capacity_1, b, moves + 1),
        (a, b) if a + b <= capacity_2 => {
            pour_from_small_to_big(capacity_1, capacity_2, goal, 0, a + b, moves + 1)
        }
        (a, b) if a - goal + b == capacity_2 => {
            pour_from_small_to_big(capacity_1, capacity_2, goal, goal, a + b - goal, moves + 1)
        }
        _ => None,
    }
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let is_small_to_big_direction = match start_bucket {
        Bucket::One => capacity_1 < capacity_2,
        Bucket::Two => capacity_2 < capacity_1,
    };

    match is_small_to_big_direction {
        true => pour_from_small_to_big(capacity_1, capacity_2, goal, 0, 0, 0),
        false => pour_from_big_to_small(),
    }
}

#[test]
fn test_case_1() {
    assert_eq!(
        solve(3, 5, 1, &Bucket::One),
        Some(BucketStats {
            moves: 4,

            goal_bucket: Bucket::One,

            other_bucket: 5,
        })
    );
}
