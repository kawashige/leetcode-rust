use std::ops::Range;

struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
    updated: Vec<(Range<i32>, Range<i32>, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self {
            rectangle,
            updated: Vec::new(),
        }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.updated
            .push((row1..row2 + 1, col1..col2 + 1, new_value));
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        if let Some((_, _, value)) =
            self.updated.iter().rev().find(|(row_range, col_range, _)| {
                row_range.contains(&row) && col_range.contains(&col)
            })
        {
            *value
        } else {
            self.rectangle[row as usize][col as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1476() {
        let mut obj = SubrectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);
        assert_eq!(obj.get_value(0, 2), 1);
        obj.update_subrectangle(0, 0, 3, 2, 5);
        assert_eq!(obj.get_value(0, 2), 5);
        assert_eq!(obj.get_value(3, 1), 5);
        obj.update_subrectangle(3, 0, 3, 2, 10);
        assert_eq!(obj.get_value(3, 1), 10);
        assert_eq!(obj.get_value(0, 2), 5);

        let mut obj = SubrectangleQueries::new(vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]]);
        assert_eq!(obj.get_value(0, 0), 1);
        obj.update_subrectangle(0, 0, 2, 2, 100);
        assert_eq!(obj.get_value(0, 0), 100);
        assert_eq!(obj.get_value(2, 2), 100);
        obj.update_subrectangle(1, 1, 2, 2, 20);
        assert_eq!(obj.get_value(2, 2), 20);
    }
}
