pub struct Solution {}

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut freq = vec![vec![vec![0; 3]; grid[0].len()]; grid.len()];
        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                freq[i][j][match grid[i][j] {
                    'X' => 0,
                    'Y' => 1,
                    '.' => 2,
                    _ => unreachable!(),
                }] += 1;
                for k in 0..2 {
                    if 0 < i {
                        freq[i][j][k] += freq[i - 1][j][k];
                    }
                    if 0 < j {
                        freq[i][j][k] += freq[i][j - 1][k];
                    }
                    if 0 < i && 0 < j {
                        freq[i][j][k] -= freq[i - 1][j - 1][k];
                    }
                }
                if freq[i][j][0] == freq[i][j][1] && 0 < freq[i][j][0] {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3212() {
        assert_eq!(
            Solution::number_of_submatrices(vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']]),
            3
        );
        assert_eq!(
            Solution::number_of_submatrices(vec![vec!['X', 'X'], vec!['X', 'Y']]),
            0
        );
        assert_eq!(
            Solution::number_of_submatrices(vec![vec!['.', '.'], vec!['.', '.']]),
            0
        );
    }
}
