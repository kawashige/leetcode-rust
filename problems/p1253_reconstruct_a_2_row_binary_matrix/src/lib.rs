pub struct Solution {}

impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; colsum.len()], vec![0; colsum.len()]];
        let mut colsum_indices = colsum.iter().enumerate().collect::<Vec<_>>();
        colsum_indices.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let mut upper_remains = upper;
        let mut lower_remains = lower;
        for (index, colsum) in colsum_indices {
            if colsum == &2 {
                result[0][index] = 1;
                result[1][index] = 1;
                upper_remains -= 1;
                lower_remains -= 1;
            } else if colsum == &1 {
                if upper_remains > 0 {
                    result[0][index] = 1;
                    upper_remains -= 1;
                } else if lower_remains > 0 {
                    result[1][index] = 1;
                    lower_remains -= 1;
                } else {
                    return Default::default();
                }
            }
        }

        if upper_remains != 0 || lower_remains != 0 {
            Default::default()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1253() {
        assert_eq!(
            Solution::reconstruct_matrix(2, 1, vec![1, 1, 1]),
            vec![vec![1, 1, 0], vec![0, 0, 1]]
        );
        assert_eq!(
            Solution::reconstruct_matrix(2, 3, vec![2, 2, 1, 1]),
            vec![] as Vec<Vec<i32>>
        );
        assert_eq!(
            Solution::reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
            vec![
                vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0],
                vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 1]
            ]
        );
    }
}
