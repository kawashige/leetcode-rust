pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();

        if matrix.len() > 0 {
            let mut start = (0, 0);
            let mut end = (matrix.len() - 1, matrix[0].len() - 1);
            while start.0 <= end.0 && start.1 <= end.1 {
                if start == end {
                    result.push(matrix[start.0][start.1]);
                    break;
                }
                if start.0 == end.0 {
                    for j in start.1..=end.1 {
                        result.push(matrix[start.0][j]);
                    }
                    break;
                }
                if start.1 == end.1 {
                    for i in start.0..=end.0 {
                        result.push(matrix[i][start.1]);
                    }
                    break;
                }
                for j in start.1..end.1 {
                    result.push(matrix[start.0][j]);
                }
                for i in start.0..end.0 {
                    result.push(matrix[i][end.1]);
                }
                for j in ((start.1 + 1)..=end.1).rev() {
                    result.push(matrix[end.0][j]);
                }
                for i in ((start.1 + 1)..=end.0).rev() {
                    result.push(matrix[i][start.1]);
                }
                start = (start.0 + 1, start.1 + 1);
                end = (end.0.saturating_sub(1), end.1.saturating_sub(1));
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0054() {
        let input1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(input1)
        );

        let input2 = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            Solution::spiral_order(input2)
        );

        let input3 = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];
        assert_eq!(
            vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8],
            Solution::spiral_order(input3)
        );
        let result: Vec<i32> = Vec::new();
        assert_eq!(result, Solution::spiral_order(vec![]));
        assert_eq!(vec![1], Solution::spiral_order(vec![vec![1]]));
        assert_eq!(vec![3, 2], Solution::spiral_order(vec![vec![3], vec![2]]));
        assert_eq!(
            vec![3, 2, 1],
            Solution::spiral_order(vec![vec![3], vec![2], vec![1]])
        );
        assert_eq!(vec![3, 2], Solution::spiral_order(vec![vec![3, 2]]));
        assert_eq!(vec![6, 9, 7], Solution::spiral_order(vec![vec![6, 9, 7]]));
        let input4 = vec![
            vec![1, 11],
            vec![2, 12],
            vec![3, 13],
            vec![4, 14],
            vec![5, 15],
            vec![6, 16],
            vec![7, 17],
            vec![8, 18],
            vec![9, 19],
            vec![10, 20],
        ];
        assert_eq!(
            vec![1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 10, 9, 8, 7, 6, 5, 4, 3, 2],
            Solution::spiral_order(input4)
        );
        let input5 = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        ];
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11],
            Solution::spiral_order(input5)
        );
    }
}
