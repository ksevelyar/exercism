mod armstrong_number;
mod difference_of_squares;
mod reverse_string;

fn main() {
    dbg!(armstrong_number::is_armstrong_number(153));
    dbg!(reverse_string::reverse("hurma"));
    dbg!(difference_of_squares::difference(10));
}
