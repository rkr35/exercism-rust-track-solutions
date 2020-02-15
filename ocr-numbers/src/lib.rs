#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

fn read(top: &str, middle: &str, bottom: &str) -> &'static str {
    match [top, middle, bottom] {
        [
            " _ ",
            "| |",
            "|_|",
        ] => "0",

        [
            "   ",
            "  |",
            "  |",
        ] => "1",

        [
            " _ ",
            " _|",
            "|_ ",
        ] => "2",

        [
            " _ ",
            " _|",
            " _|",
        ] => "3",

        [
            "   ",
            "|_|",
            "  |",
        ] => "4",

        [
            " _ ",
            "|_ ",
            " _|",
        ] => "5",

        [
            " _ ",
            "|_ ",
            "|_|",
        ] => "6",

        [
            " _ ",
            "  |",
            "  |",
        ] => "7",

        [
            " _ ",
            "|_|",
            "|_|",
        ] => "8",

        [
            " _ ",
            "|_|",
            " _|",
        ] => "9",

        _ => "?",
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    const DIGIT_WIDTH: usize = 3;
    const DIGIT_HEIGHT: usize = 4;

    let lines: Vec<_> = input.split('\n').collect();
    
    if lines.is_empty() {
        return Ok("".into());
    }

    let num_rows = lines.len();

    if num_rows % DIGIT_HEIGHT != 0 {
        return Err(Error::InvalidRowCount(num_rows));
    }

    let bad_num_columns = lines
        .iter()
        .map(|l| l.len())
        .find(|c| c % DIGIT_WIDTH != 0);

    if let Some(c) = bad_num_columns {
        return Err(Error::InvalidColumnCount(c));
    }
    
    let num_digits = lines[0].len() / DIGIT_WIDTH;

    let mut s: String = lines
        .chunks_exact(DIGIT_HEIGHT)
        .flat_map(|chunk|
            (0..num_digits)
                .map(move |i| {
                    let start = i * DIGIT_WIDTH;
                    let end = (i + 1) * DIGIT_WIDTH;

                    let top = &chunk[0][start..end];
                    let middle = &chunk[1][start..end];
                    let bottom = &chunk[2][start..end];

                    read(top, middle, bottom)
                })
                .chain(std::iter::once(","))
        )
        .collect();

    s.pop();

    Ok(s)
}