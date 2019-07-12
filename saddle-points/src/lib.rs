pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec![];

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, element) in row.iter().enumerate() {
            // Is this element the largest in its row?
            if element != row.iter().max().expect("Found an empty row when taking a max()") {
                continue;
            }

            // Is this element the smallest in its column?
            let column = (0..input.len()).map(|row_index| input[row_index][column_index]);

            if *element != column.min().expect("Found an empty column when taking a min()") {
                continue;
            }

            saddle_points.push((row_index, column_index));
        }
    }

    saddle_points
}
