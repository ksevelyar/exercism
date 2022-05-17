mod armstrong_number;
mod bob;
mod difference_of_squares;
mod magazine_cutout;
mod reverse_string;

fn main() {
    dbg!(armstrong_number::is_armstrong_number(153));
    dbg!(reverse_string::reverse("hurma"));
    dbg!(difference_of_squares::difference(10));
    dbg!(bob::reply("BABAH?"));

    let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "Amy Mainzer chatting with Leonardo DiCaprio"
        .split_whitespace()
        .collect::<Vec<&str>>();
    dbg!(magazine_cutout::can_construct_note(&magazine, &note));
}
