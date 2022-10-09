use std::collections::HashMap;

#[derive(Debug)]
struct Team {
    matches_played: u32,
    matches_won: u32,
    matches_drawn: u32,
    matches_lost: u32,
    points: u32,
}

fn row(name: &str, team: &Team) -> String {
    format!(
        "{:31}|  {} |  {} |  {} |  {} |  {}",
        name,
        team.matches_played,
        team.matches_won,
        team.matches_drawn,
        team.matches_lost,
        team.points
    )
}

fn header() -> &'static str {
    "Team                           | MP |  W |  D |  L |  P"
}

fn parse_matches(results: &str) -> Vec<Vec<&str>> {
    results
        .lines()
        .map(|result| result.split(';').collect::<Vec<&str>>())
        .collect()
}

fn new_team() -> Team {
    Team {
        matches_played: 0,
        matches_won: 0,
        matches_drawn: 0,
        matches_lost: 0,
        points: 0,
    }
}

fn add_win(teams: &mut HashMap<String, Team>, team1_name: &str, team2_name: &str) {
    let team1 = teams.entry(team1_name.to_owned()).or_insert_with(new_team);
    team1.matches_played += 1;
    team1.matches_won += 1;
    team1.points += 3;

    let team2 = teams.entry(team2_name.to_owned()).or_insert_with(new_team);
    team2.matches_played += 1;
    team2.matches_lost += 1;
}

fn add_draw(teams: &mut HashMap<String, Team>, team1_name: &str, team2_name: &str) {
    let team1 = teams.entry(team1_name.to_owned()).or_insert_with(new_team);
    team1.matches_played += 1;
    team1.matches_drawn += 1;
    team1.points += 1;

    let team2 = teams.entry(team2_name.to_owned()).or_insert_with(new_team);
    team2.matches_played += 1;
    team2.matches_drawn += 1;
    team2.points += 1;
}

fn teams(input: Vec<Vec<&str>>) -> HashMap<String, Team> {
    input
        .iter()
        .fold(HashMap::new(), |mut teams, match_result| {
            let team1 = match_result[0];
            let team2 = match_result[1];

            match match_result[2] {
                "win" => add_win(&mut teams, team1, team2),
                "loss" => add_win(&mut teams, team2, team1),
                "draw" => add_draw(&mut teams, team1, team2),
                _ => panic!("The result of the match is an unknown word."),
            };

            teams
        })
}

pub fn tally(match_results: &str) -> String {
    if match_results.is_empty() {
        return header().to_owned();
    }

    let input = parse_matches(match_results);
    let teams = teams(input);

    let mut teams_vec: Vec<(String, Team)> = teams.into_iter().collect();
    teams_vec.sort_unstable_by_key(|item| (-(item.1.points as i32), item.0.clone()));

    let teams_string: String = teams_vec
        .iter()
        .map(|(name, team)| row(name, team))
        .collect::<Vec<String>>()
        .join("\n");

    format!("{}\n{}", header(), teams_string)
}
