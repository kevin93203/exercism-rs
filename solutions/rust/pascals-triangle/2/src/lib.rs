pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let count = self.row_count as usize;
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(count);

        for i in 0..count {
            let mut row = Vec::with_capacity(i + 1);
            row.push(1);

            if i > 0 {
                let prev_row = &rows[i - 1];
                for pair in prev_row.windows(2) {
                    row.push(pair[0] + pair[1]);
                }
                row.push(1);
            }
            rows.push(row);
        }
        rows
    }
}