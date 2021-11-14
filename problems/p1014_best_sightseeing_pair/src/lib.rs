pub struct Solution {}

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_pair = values[0];

        for i in 1..values.len() {
            max = max.max(max_pair + values[i] - i as i32);
            max_pair = max_pair.max(values[i] + i as i32)
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1014() {
        assert_eq!(
            Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]),
            11
        );
        assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
    }
}
