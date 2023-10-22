pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(_max_weight: u32, _items: Vec<Item>) -> u32 {
    unimplemented!("Solve the knapsack exercise");
}

#[ignore]
#[test]
fn test_example_knapsack() {
    let max_weight = 10;

    let items = vec![
        Item {
            weight: 5,

            value: 10,
        },
        Item {
            weight: 4,

            value: 40,
        },
        Item {
            weight: 6,

            value: 30,
        },
        Item {
            weight: 4,

            value: 50,
        },
    ];

    assert_eq!(maximum_value(max_weight, items), 90);
}
