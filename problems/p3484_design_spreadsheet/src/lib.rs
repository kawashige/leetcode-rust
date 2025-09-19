struct Spreadsheet {
    sheet: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            sheet: vec![vec![0; 26]; rows as usize],
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let (row, column) = self.get_cell_indices(&cell);
        self.sheet[row][column] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        let (row, column) = self.get_cell_indices(&cell);
        self.sheet[row][column] = 0;
    }

    fn get_value(&self, formula: String) -> i32 {
        let vals = formula[1..]
            .split('+')
            .map(|v| v.to_string())
            .collect::<Vec<_>>();
        let v1 = if (b'A'..=b'Z').contains(&vals[0].as_bytes()[0]) {
            let (row, column) = self.get_cell_indices(&vals[0]);
            self.sheet[row][column]
        } else {
            vals[0].parse::<i32>().unwrap()
        };
        let v2 = if (b'A'..=b'Z').contains(&vals[1].as_bytes()[0]) {
            let (row, column) = self.get_cell_indices(&vals[1]);
            self.sheet[row][column]
        } else {
            vals[1].parse::<i32>().unwrap()
        };
        v1 + v2
    }

    fn get_cell_indices(&self, cell: &str) -> (usize, usize) {
        let column = (cell.as_bytes()[0] - b'A') as usize;
        let row = cell[1..].parse::<usize>().unwrap() - 1;
        (row, column)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3484() {
        let mut obj = Spreadsheet::new(3);
        assert_eq!(obj.get_value("=5+7".to_string()), 12);
        obj.set_cell("A1".to_string(), 10);
        assert_eq!(obj.get_value("=A1+6".to_string()), 16);
        obj.set_cell("B2".to_string(), 15);
        assert_eq!(obj.get_value("=A1+B2".to_string()), 25);
        obj.reset_cell("A1".to_string());
        assert_eq!(obj.get_value("=A1+B2".to_string()), 15);
    }
}
