#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec![];

    // row_maxes[i] contains the maximum element of row i.
    let mut row_maxes = vec![None; input.len()];

    let num_columns = input.get(0).map_or(0, Vec::len);
    
    // column_mins[i] contains the minimum element of column i.
    let mut column_mins = vec![None; num_columns];

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, element) in row.iter().enumerate() {
            // A closure that returns the maximum element for this row.
            // Used to cache this maximum element into row_maxes[row_index].
            let get_max = || *row
                .iter()
                .max()
                .expect("Empty row when calling max()");

            // Get the maximum element for this row either from the cache or from executing the closure.
            let row_max = row_maxes[row_index].get_or_insert_with(get_max);

            // Is this element the largest in its row?
            if element != row_max {
                // No. By definition, this element cannot be a saddle point then.
                continue;
            }

            // A closure that returns the minimum element for this column.
            // Used to cache this minimum element into column_mins[column_index].
            let get_min = || (0..input.len())
                .map(|row_index| input[row_index][column_index])
                .min()
                .expect("Empty column when calling min()"); 

            // Get the minimum element for this column either from the cache or from executing the closure.
            let column_min = column_mins[column_index].get_or_insert_with(get_min);

            // Is this element the smallest in its column?
            if element != column_min {
                // No. By definition, this element cannot be a saddle point then.
                continue;
            }

            saddle_points.push((row_index, column_index));
        }
    }

    saddle_points
}
