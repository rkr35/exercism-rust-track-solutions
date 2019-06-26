type Row = Vec<u32>;
type Rows = Vec<Row>;

pub struct PascalsTriangle {
    rows: Rows,
}

impl PascalsTriangle {
    pub fn new(number_of_rows: usize) -> Self {
        Self {
            rows: Vec::with_capacity(number_of_rows),
        }
    }

    pub fn rows(&self) -> Rows {
        self.rows.clone()
    }
}