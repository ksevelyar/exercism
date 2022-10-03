fn header() -> &'static str {
    "Team                           | MP |  W |  D |  L |  P"
}

pub fn tally(match_results: &str) -> String {
    dbg!(match_results.split("\n"));

    header().to_string()
}
