#![warn(clippy::pedantic)]
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Default)]
struct Stats {
    matches: u8,
    won: u8,
    drew: u8,
    loss: u8,
    points: u16,
}

type AllStatsInternal<'a> = HashMap<&'a str, Stats>;

#[derive(Default)]
struct AllStats<'a>(AllStatsInternal<'a>);

impl<'a> AllStats<'a> {
    fn update_team(&mut self, team: &'a str, conclusion: &'a str) {
        self.0.entry(team).or_default().update(conclusion);
    }

    fn update(&mut self, first_team: &'a str, second_team: &'a str, conclusion: &'a str) {
        self.update_team(first_team, conclusion);

        self.update_team(second_team, match conclusion {
            "win" => "loss",
            "loss" => "win",
            "draw" => "draw",
            _ => unreachable!("Encountered unknown conclusion when finding opposite (\"{}\")", conclusion),
        });
    }
}

impl<'a> IntoIterator for AllStats<'a> {
    type Item = <AllStatsInternal<'a> as IntoIterator>::Item;
    type IntoIter = <AllStatsInternal<'a> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
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

macro_rules! row_format {
    ($($arg:expr),+) => {
        format!("{:31}|{:>width$} |{:>width$} |{:>width$} |{:>width$} |{:>width$}", $($arg,)+ width=3)
    };
}

fn get_header() -> String {
    row_format!("Team", "MP", "W", "D", "L", "P")
}

pub fn tally(match_results: &str) -> String {
    if match_results.is_empty() {
        return get_header();
    }

    let mut pieces = match_results.split(';').flat_map(|s| s.split('\n'));

    let mut line: [&str; 3] = Default::default();
    let mut line_cursor = 0;

    let mut team_stats: AllStats = Default::default();

    while let Some(piece) = pieces.next() {
        line[line_cursor] = piece;
        line_cursor += 1;

        if line_cursor == line.len() {
            let [first_team, second_team, conclusion] = line;
            team_stats.update(first_team, second_team, conclusion);
            line = Default::default();
            line_cursor = 0;
        }
    }

    let mut team_stats: Vec<_> = team_stats.into_iter().collect();

    team_stats.sort_unstable_by(|(name1, stats1), (name2, stats2)| {
        match stats2.points.cmp(&stats1.points) {
            Ordering::Equal => name1.cmp(name2),
            points_ordering => points_ordering,
        }
    });

    get_header()
        + "\n"
        + &team_stats
            .into_iter()
            .map(|(name, stats)| {
                row_format!(
                    name,
                    stats.matches,
                    stats.won,
                    stats.drew,
                    stats.loss,
                    stats.points
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
}
