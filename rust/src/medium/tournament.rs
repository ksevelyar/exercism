struct Team {
    name: String,
    matches_played: u32,
    matches_won: u32,
    matches_drawn: u32,
    matches_lost: u32,
    points: u32,
}

fn header() -> &'static str {
    "Team                           | MP |  W |  D |  L |  P"
}

fn parse_matches(results: &str) -> Vec<Vec<&str>> {
    results
        .split("\n")
        .map(|result| result.split(";").collect::<Vec<&str>>())
        .collect()
}

pub fn tally(match_results: &str) -> String {
    dbg!(parse_matches(match_results));

    header().to_string()
}
