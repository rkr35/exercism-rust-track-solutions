type Row = Vec<u32>;
type Rows = Vec<Row>;

// todo: Revisit when `const fn` lands in stable.
fn falling_factorial(n: usize, k: usize) -> usize {
    ((n-k+1) ..= n).product()
}

fn binomial_coefficient(n: usize, k: usize) -> usize {
    let non_repeating_permutations = falling_factorial(n, k);
    let combinations_per_permutation: usize = (1 ..= k).product();
    non_repeating_permutations / combinations_per_permutation
}

pub struct PascalsTriangle {
    rows: Rows,
}

fn calculate_row(row: usize) -> Row {
    (0 ..= row)
        .map(|column| binomial_coefficient(row, column) as u32)
        .collect()
}

impl PascalsTriangle {
    pub fn new(number_of_rows: usize) -> Self {
        Self {
            rows: (0 .. number_of_rows).map(calculate_row).collect(),
        }
    }

    pub fn rows(&self) -> Rows {
        self.rows.clone()
    }
}