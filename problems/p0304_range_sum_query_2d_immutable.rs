struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() {
            return Self {
                sums: vec![vec![0]],
            };
        }
        let mut sums = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            let mut row_sum = 0;
            for j in 0..matrix[0].len() {
                row_sum += matrix[i][j];
                sums[i][j] = row_sum;
                if 0 < i {
                    sums[i][j] += sums[i - 1][j];
                }
            }
        }
        Self { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut result = self.sums[row2 as usize][col2 as usize];
        if 0 < row1 {
            result -= self.sums[row1 as usize - 1][col2 as usize];
        }
        if 0 < col1 {
            result -= self.sums[row2 as usize][col1 as usize - 1];
        }
        if 0 < row1 && 0 < col1 {
            result += self.sums[row1 as usize - 1][col1 as usize - 1];
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0304() {
        let obj = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(8, obj.sum_region(2, 1, 4, 3));
        assert_eq!(11, obj.sum_region(1, 1, 2, 2));
        assert_eq!(12, obj.sum_region(1, 2, 2, 4));

        let obj = NumMatrix::new(vec![]);
        assert_eq!(0, obj.sum_region(0, 0, 0, 0));
    }
}
