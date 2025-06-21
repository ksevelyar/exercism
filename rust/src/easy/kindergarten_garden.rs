const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let start = student_to_index(student) * 2;
    let end = start + 1;

    diagram
        .lines()
        .flat_map(|line| (line[start..=end]).chars().map(char_to_plant))
        .collect()
}

fn student_to_index(student: &str) -> usize {
    STUDENTS
        .iter()
        .position(|elem| *elem == student)
        .expect("An unknown student 🐗")
}

fn char_to_plant(ch: char) -> &'static str {
    match ch {
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        'V' => "violets",
        _ => panic!("An unknown plant 🐗"),
    }
}

#[test]
fn test_garden_with_single_student() {
    let diagram = "RC
GG";
    let student = "Alice";
    let expected = vec!["radishes", "clover", "grass", "grass"];

    assert_eq!(plants(diagram, student), expected);
}

#[test]
fn test_for_bob_second_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";

    let student = "Bob";

    let expected = vec!["clover", "grass", "clover", "clover"];

    assert_eq!(plants(diagram, student), expected);
}
