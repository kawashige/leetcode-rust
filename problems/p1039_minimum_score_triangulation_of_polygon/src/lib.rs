pub struct Solution {}

impl Solution {
    pub fn recurse(values: &Vec<i32>, s: usize, e: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if e - s + 1 < 3 {
            0
        } else if memo[s][e] > 0 {
            memo[s][e]
        } else {
            let score = ((s + 1)..e)
                .map(|i| {
                    values[s] * values[e] * values[i]
                        + Self::recurse(values, s, i, memo)
                        + Self::recurse(values, i, e, memo)
                })
                .min()
                .unwrap();
            memo[s][e] = score;
            score
        }
    }

    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        Self::recurse(
            &values,
            0,
            values.len() - 1,
            &mut vec![vec![0; values.len()]; values.len()],
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1039() {
        assert_eq!(Solution::min_score_triangulation(vec![1, 2, 3]), 6);
        assert_eq!(Solution::min_score_triangulation(vec![3, 7, 4, 5]), 144);
        assert_eq!(
            Solution::min_score_triangulation(vec![1, 3, 1, 4, 1, 5]),
            13
        );
    }
}
