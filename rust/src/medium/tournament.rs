use std::collections::HashMap;

#[derive(Debug)]
struct Team {
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

fn teams(input: Vec<Vec<&str>>) -> HashMap<String, Team> {
    input.iter().fold(HashMap::new(), |mut acc, result| {
        let team1 = result[0];
        let team2 = result[1];

        acc.entry(team1.to_owned()).or_insert(Team {
            matches_played: 0,
            matches_won: 0,
            matches_drawn: 0,
            matches_lost: 0,
            points: 0,
        });

        acc.entry(team2.to_owned()).or_insert(Team {
            matches_played: 0,
            matches_won: 0,
            matches_drawn: 0,
            matches_lost: 0,
            points: 0,
        });

        acc
    })
}

pub fn tally(match_results: &str) -> String {
    let input = parse_matches(match_results);
    dbg!(teams(input));

    header().to_string()
}
