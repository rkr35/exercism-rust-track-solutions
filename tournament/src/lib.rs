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

            _ => unimplemented!(
                "update() not implemented for conclusion of \"{}\"",
                conclusion
            ),
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

    let mut line: [Option<&str>; 3] = Default::default();
    let mut line_cursor = 0;

    let mut team_stats: HashMap<_, Stats> = HashMap::new();

    while let Some(piece) = pieces.next() {
        line[line_cursor] = Some(piece);
        line_cursor += 1;

        if line_cursor != line.len() {
            continue;
        }

        let [first_team, second_team, conclusion] = line;

        let [first_team, second_team, conclusion] = [
            first_team.unwrap(),
            second_team.unwrap(),
            conclusion.unwrap(),
        ];

        let opposite_conclusion = match conclusion {
            "win" => "loss",
            "loss" => "win",
            "draw" => "draw",

            _ => unreachable!(
                "Encountered unknown conclusion when finding opposite (\"{}\")",
                conclusion
            ),
        };

        team_stats
            .entry(first_team)
            .or_default()
            .update(conclusion);

        team_stats
            .entry(second_team)
            .or_default()
            .update(opposite_conclusion);

        line = Default::default();
        line_cursor = 0;
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
