#![warn(clippy::pedantic)]
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    if match_results.is_empty() { 
        get_header() 
    }
    else {
        get_header() + "\n" + &get_stat_rows(match_results).join("\n") 
    }
}

macro_rules! row_format {
    ($($arg:expr),+) => {
        format!("{:31}|{:>width$} |{:>width$} |{:>width$} |{:>width$} |{:>width$}", $($arg,)+ width=3)
    };
}

fn get_header() -> String {
    row_format!("Team", "MP", "W", "D", "L", "P")
}

#[derive(Default)]
struct Stats {
    matches: u8,
    won: u8,
    drew: u8,
    loss: u8,
    points: u16,
}

impl Stats {
    fn update(&mut self, conclusion: &str) {
        match conclusion {
            "win" => { self.won += 1; self.points += 3; }
            "draw" => { self.drew += 1; self.points += 1; }
            "loss" => self.loss += 1,
            _ => unimplemented!("update() not implemented for conclusion of \"{}\"", conclusion),
        };

        self.matches += 1;
    }
}

fn get_stat_rows(match_results: &str) -> Vec<String> {
    let mut team_stats: Vec<_> = parse_team_stats(match_results).into_iter().collect();

    // Sort by points and then team name.
    team_stats.sort_unstable_by(|(name1, stats1), (name2, stats2)| {
        match stats2.points.cmp(&stats1.points) {
            Ordering::Equal => name1.cmp(name2),
            points_ordering => points_ordering,
        }
    });

    team_stats
        .into_iter()
        .map(|(name, stats)| row_format!(name, stats.matches, stats.won, stats.drew, stats.loss, stats.points))
        .collect::<Vec<_>>()
}

type AllStats<'a> = HashMap<&'a str, Stats>;

fn parse_team_stats(match_results: &str) -> AllStats {
    fn update<'a>(stats: &mut AllStats<'a>, first_team: &'a str, second_team: &'a str, conclusion: &str) {
        fn update_team<'a>(stats: &mut AllStats<'a>, team: &'a str, conclusion: &str) {
            stats.entry(team).or_default().update(conclusion);
        }

        update_team(stats, first_team, conclusion);

        update_team(stats, second_team, match conclusion {
            "win" => "loss",
            "loss" => "win",
            "draw" => "draw",
            _ => unreachable!("Encountered unknown conclusion when finding opposite (\"{}\")", conclusion),
        });
    }

    let mut line: [&str; 3] = Default::default();
    let mut team_stats: AllStats = Default::default();

    for (mut cursor, piece) in match_results.split(';').flat_map(|s| s.split('\n')).enumerate() {
        cursor %= line.len();
        line[cursor] = piece;

        if cursor + 1 == line.len() {
            let [first_team, second_team, conclusion] = line;
            update(&mut team_stats, first_team, second_team, conclusion);
        }
    }

    team_stats
}