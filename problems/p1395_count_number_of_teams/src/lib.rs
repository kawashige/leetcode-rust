pub struct Solution {}

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        (1..rating.len() - 1)
            .map(|j| {
                (0..j).filter(|i| rating[*i] < rating[j]).count()
                    * (j..rating.len()).filter(|k| rating[j] < rating[*k]).count()
                    + (0..j).filter(|i| rating[*i] > rating[j]).count()
                        * (j..rating.len()).filter(|k| rating[j] > rating[*k]).count()
            })
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1395() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
        assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
        assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
    }
}
