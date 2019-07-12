#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec![];
    let mut row_maxes = vec![None; input.len()];
    let num_columns = input.get(0).map_or(0, Vec::len);
    let mut column_mins = vec![None; num_columns];

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, element) in row.iter().enumerate() {
            // Is this element the largest in its row?
            let get_max = || *row
                .iter()
                .max()
                .expect("Empty row when calling max()");

            let row_max = row_maxes[row_index].get_or_insert_with(get_max);

            if element != row_max {
                continue;
            }

            // Is this element the smallest in its column?
            let get_min = || (0..input.len())
                .map(|row_index| input[row_index][column_index])
                .min()
                .expect("Empty column when calling min()"); 

            let column_min = column_mins[column_index].get_or_insert_with(get_min);

            if element != column_min {
                continue;
            }

            saddle_points.push((row_index, column_index));
        }
    }

    saddle_points
}
