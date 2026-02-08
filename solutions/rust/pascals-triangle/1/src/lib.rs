pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        for i in 0..(self.row_count as usize) {
            let mut row = Vec::new();
            for j in 0..=i {
                if i ==0 || j == 0 || j ==i {
                    row.push(1);
                }
                else {
                    row.push(rows[i-1][j-1] + rows[i-1][j]);
                }
            }
            rows.push(row);
        }
        rows
    }
}
