#[derive(Copy, Clone)]
enum Actions {
    Wink = 0b0001,
    DoubleBlink = 0b00010,
    CloseYourEyes = 0b00100,
    Jump = 0b01000,
    Reverse = 0b10000,
}

const ACTIONS: [Actions; 4] = [
    Actions::Wink,
    Actions::DoubleBlink,
    Actions::CloseYourEyes,
    Actions::Jump,
];

pub fn actions(n: u8) -> Vec<&'static str> {
    let message = ACTIONS.iter().flat_map(|action| match action {
        Actions::Wink if n & *action as u8 != 0 => Some("wink"),
        Actions::DoubleBlink if n & *action as u8 != 0 => Some("double blink"),
        Actions::CloseYourEyes if n & *action as u8 != 0 => Some("close your eyes"),
        Actions::Jump if n & *action as u8 != 0 => Some("jump"),
        _ => None,
    });

    match n & Actions::Reverse as u8 != 0 {
        true => message.rev().collect(),
        false => message.collect(),
    }
}

#[test]
fn wink_for_1() {
    assert_eq!(actions(1), vec!["wink"])
}

#[test]
fn double_blink_for_10() {
    assert_eq!(actions(2), vec!["double blink"])
}

#[test]
fn close_your_eyes_for_100() {
    assert_eq!(actions(4), vec!["close your eyes"])
}

#[test]
fn jump_for_1000() {
    assert_eq!(actions(8), vec!["jump"])
}

#[test]
fn combine_two_actions() {
    assert_eq!(actions(3), vec!["wink", "double blink"])
}

#[test]
fn reverse_two_actions() {
    assert_eq!(actions(19), vec!["double blink", "wink"])
}

#[test]
fn reversing_one_action_gives_the_same_action() {
    assert_eq!(actions(24), vec!["jump"])
}

#[test]
fn reversing_no_actions_still_gives_no_actions() {
    assert_eq!(actions(16), Vec::<&'static str>::new())
}

#[test]
fn all_possible_actions() {
    assert_eq!(
        actions(15),
        vec!["wink", "double blink", "close your eyes", "jump"]
    )
}

#[test]
fn reverse_all_possible_actions() {
    assert_eq!(
        actions(31),
        vec!["jump", "close your eyes", "double blink", "wink"]
    )
}

#[test]
fn do_nothing_for_zero() {
    assert_eq!(actions(0), Vec::<&'static str>::new())
}
