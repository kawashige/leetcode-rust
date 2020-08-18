pub struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row0 = false;
        let mut column0 = false;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    if i == 0 || j == 0 {
                        if i == 0 {
                            row0 = true;
                        }
                        if j == 0 {
                            column0 = true;
                        }
                    } else {
                        matrix[i][0] = 0;
                        matrix[0][j] = 0;
                    }
                }
            }
        }

        for i in 1..matrix[0].len() {
            if matrix[0][i] == 0 {
                for j in 1..matrix.len() {
                    matrix[j][i] = 0;
                }
            }
        }

        for i in 1..matrix.len() {
            if matrix[i][0] == 0 {
                for j in 1..matrix[0].len() {
                    matrix[i][j] = 0;
                }
            }
        }

        if column0 {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        }
        if row0 {
            for i in 0..matrix[0].len() {
                matrix[0][i] = 0;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0073() {
        let mut input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut input);
        assert_eq!(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]], input);

        let mut input2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut input2);
        assert_eq!(
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
            input2
        );

        let mut input3 = vec![vec![1]];
        Solution::set_zeroes(&mut input3);
        assert_eq!(vec![vec![1]], input3);

        let mut input4 = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![0, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        Solution::set_zeroes(&mut input4);
        assert_eq!(
            vec![
                vec![0, 0, 3, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0]
            ],
            input4
        );

        let mut input5 = vec![vec![1, 0, 3]];
        Solution::set_zeroes(&mut input5);
        assert_eq!(vec![vec![0, 0, 0]], input5);
    }
}
