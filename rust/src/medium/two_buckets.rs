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

fn pour_from_big_to_small(
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

    if b2 == goal {
        return Some(BucketStats {
            moves,
            goal_bucket: Bucket::Two,
            other_bucket: b1,
        });
    }

    match (b1, b2) {
        (a, 0) => {
            println!("🧵 1");
            pour_from_big_to_small(capacity_1, capacity_2, goal, a, capacity_2, moves + 1)
        }
        (0, b) if b > capacity_1 => {
            println!("🧵 2");
            pour_from_big_to_small(
                capacity_1,
                capacity_2,
                goal,
                capacity_1,
                b - capacity_1,
                moves + 1,
            )
        }
        (0, b) => {
            println!("🧵 3");
            pour_from_big_to_small(capacity_1, capacity_2, goal, b, 0, moves + 1)
        }
        (a, b) if a == capacity_1 => {
            println!("🧵 4");
            pour_from_big_to_small(capacity_1, capacity_2, goal, 0, b, moves + 1)
        }
        (a, b) if a < capacity_1 => {
            println!("🧵 5");
            pour_from_big_to_small(
                capacity_1,
                capacity_2,
                goal,
                capacity_1,
                b - (capacity_1 - a),
                moves + 1,
            )
        }
        _ => None,
    }
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

    if b2 == goal {
        return Some(BucketStats {
            moves,
            goal_bucket: Bucket::Two,
            other_bucket: b1,
        });
    }

    match (b1, b2) {
        (a, b) if a == capacity_1 && b == capacity_2 => None,
        (0, b) => pour_from_small_to_big(capacity_1, capacity_2, goal, capacity_1, b, moves + 1),
        (a, 0) if capacity_2 == goal => {
            pour_from_small_to_big(capacity_1, capacity_2, goal, a, capacity_2, moves + 1)
        }
        (a, b) if a + b <= capacity_2 => {
            pour_from_small_to_big(capacity_1, capacity_2, goal, 0, a + b, moves + 1)
        }
        (a, b) if a + b - goal == capacity_2 => {
            pour_from_small_to_big(capacity_1, capacity_2, goal, goal, a + b - goal, moves + 1)
        }
        (a, b) if b < capacity_2 => pour_from_small_to_big(
            capacity_1,
            capacity_2,
            goal,
            a - (capacity_2 - b),
            capacity_2,
            moves + 1,
        ),
        (a, b) if b == capacity_2 => {
            pour_from_small_to_big(capacity_1, capacity_2, goal, a, 0, moves + 1)
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
    match start_bucket {
        Bucket::One => match capacity_1 <= capacity_2 {
            true => pour_from_small_to_big(capacity_1, capacity_2, goal, 0, 0, 0),
            false => pour_from_big_to_small(capacity_1, capacity_2, goal, 0, 0, 0),
        },
        Bucket::Two => match capacity_2 <= capacity_1 {
            true => pour_from_small_to_big(capacity_1, capacity_2, goal, 0, 0, 0),
            false => pour_from_big_to_small(capacity_1, capacity_2, goal, 0, 0, 0),
        },
    }
}

#[test]
fn goal_equal_to_other_bucket() {
    assert_eq!(
        solve(2, 3, 3, &Bucket::One),
        Some(BucketStats {
            moves: 2,

            goal_bucket: Bucket::Two,

            other_bucket: 2,
        })
    );
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

#[test]
fn test_case_2() {
    assert_eq!(
        solve(3, 5, 1, &Bucket::Two),
        Some(BucketStats {
            moves: 8,

            goal_bucket: Bucket::Two,

            other_bucket: 3,
        })
    );
}

#[test]
fn goal_equal_to_start_bucket() {
    assert_eq!(
        solve(1, 3, 3, &Bucket::Two),
        Some(BucketStats {
            moves: 1,

            goal_bucket: Bucket::Two,

            other_bucket: 0,
        })
    );
}

#[test]
fn with_same_buckets_but_different_goal_then_it_is_possible() {
    assert_eq!(
        solve(6, 15, 9, &Bucket::One),
        Some(BucketStats {
            moves: 10,

            goal_bucket: Bucket::Two,

            other_bucket: 0,
        })
    );
}

#[test]
fn not_possible_to_reach_the_goal() {
    assert_eq!(solve(6, 15, 5, &Bucket::One), None);
}
