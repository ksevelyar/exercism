mod medium;

fn main() {
    dbg!(medium::minesweeper::annotate(&[
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ]));
}
