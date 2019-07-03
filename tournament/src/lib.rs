#![warn(clippy::pedantic)]
use std::collections::HashMap;
use std::cmp::Ordering;

type Name = String;

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
            "win" => {
                const POINTS_FOR_WIN: u16 = 3;
                self.won += 1;
                self.points += POINTS_FOR_WIN;
            }

            "draw" => {
                const POINTS_FOR_DRAW: u16 = 1;
                self.drew += 1;
                self.points += POINTS_FOR_DRAW;
            }

            "loss" => self.loss += 1,

            _ => unimplemented!("update() not implemented for conclusion of \"{}\"", conclusion)
        };

        self.matches += 1;
    }
}

macro_rules! row_format {($($arg:expr),+) => {format!("{:31}|{:>width$} |{:>width$} |{:>width$} |{:>width$} |{:>width$}", $($arg,)+ width=3)};}

fn get_header() -> String {
    row_format!("Team", "MP", "W", "D", "L", "P")
}

pub fn tally(match_results: &str) -> String {
    if match_results.is_empty() {
        return get_header();
    }

    let mut team_stats: HashMap<Name, Stats> = HashMap::new();
    let split: Vec<_> = match_results.split(';').collect();
    
    let first_team = split[0];
    let second_team = split[1];
    let conclusion = split[2];

    let opposite_conclusion = match conclusion {
        "win" => "loss",
        "loss" => "win",
        "draw" => "draw",
        _ => unreachable!("Encountered unknown conclusion when finding opposite (\"{}\")", conclusion)
    };

    team_stats.entry(first_team.to_string()).or_default().update(conclusion);
    team_stats.entry(second_team.to_string()).or_default().update(opposite_conclusion);

    let mut team_stats: Vec<_> = team_stats.into_iter().collect();

    team_stats.sort_unstable_by(|(name1, stats1), (name2, stats2)| {
        match stats2.points.cmp(&stats1.points) {
            Ordering::Equal => name1.cmp(name2),
            points_ordering => points_ordering,
        }
    });

    get_header() + "\n" + 
    
    &team_stats
        .into_iter()
        .map(|(name, stats)| row_format!(name, stats.matches, stats.won, stats.drew, stats.loss, stats.points))
        .collect::<Vec<_>>()
        .join("\n")
}