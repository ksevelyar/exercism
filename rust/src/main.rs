mod medium;

fn main() {
    dbg!(medium::sublist::sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));

    dbg!(medium::sublist::sublist(&[1, 2, 3], &[1, 2, 3]));
    dbg!(medium::sublist::sublist(&[1, 2, 3], &[1, 4, 3]));
}
