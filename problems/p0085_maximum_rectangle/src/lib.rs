pub struct Solution {}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let mut left_ones = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut max_area = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '0' {
                    continue;
                }
                left_ones[i][j] += if j > 0 { left_ones[i][j - 1] } else { 0 } + 1;

                let mut min_w = std::i32::MAX;
                for k in (0..=i).rev().take_while(|k| left_ones[*k][j] != 0) {
                    min_w = min_w.min(left_ones[k][j]);
                    max_area = max_area.max(min_w * (i - k + 1) as i32);
                }
            }
        }

        max_area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0085() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
        assert_eq!(Solution::maximal_rectangle(vec![]), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0', '0']]), 0);
    }
}
