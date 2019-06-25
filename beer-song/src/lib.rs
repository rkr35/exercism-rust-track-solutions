fn part_a(n: i32) -> String {
    match n {
        0 => "No more bottles".to_owned(),
        _ => part_b(n),
    }
}

fn part_b(n: i32) -> String {
    match n {
        -1 => part_b(99),
        0 => "no more bottles".to_owned(),
        1 => "1 bottle".to_owned(),
        _ => format!("{} bottles", n),
    }
}

fn part_c(n: i32) -> String {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    }.to_owned()
}

pub fn verse(n: i32) -> String {
    format!("{} of beer on the wall, {} of beer.\n", part_a(n), part_b(n)) + &
    format!("{}, {} of beer on the wall.\n", part_c(n), part_b(n - 1))
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}