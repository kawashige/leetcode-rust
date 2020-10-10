pub struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        fn find_square(m: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
            let rest = std::cmp::min(m.len() - 1 - i, m[0].len() - 1 - j);
            (1..=rest)
                .find(|n| {
                    (0..*n).any(|k| m[i + n][j + k] == '0')
                        || (0..*n).any(|k| m[i + k][j + n] == '0')
                        || m[i + n][j + n] == '0'
                })
                .unwrap_or(rest + 1) as i32
        }

        let mut max = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '1' {
                    max = std::cmp::max(max, find_square(&matrix, i, j))
                }
            }
        }
        max * max
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0221() {
        assert_eq!(
            4,
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ])
        );
        assert_eq!(
            1,
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '0', '1', '1'],
                vec!['1', '1', '1', '0', '1'],
                vec!['1', '0', '0', '1', '0'],
            ])
        )
    }
}
