pub struct Solution {}

impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let mut non_increasing_days = vec![1; security.len()];
        let mut non_decreasing_days = vec![1; security.len()];

        for i in 1..security.len() {
            if security[i - 1] >= security[i] {
                non_increasing_days[i] += non_increasing_days[i - 1];
            }
        }
        for i in (0..security.len() - 1).rev() {
            if security[i] <= security[i + 1] {
                non_decreasing_days[i] += non_decreasing_days[i + 1];
            }
        }

        (0..security.len() as i32)
            .filter(|i| {
                time < non_increasing_days[*i as usize] && time < non_decreasing_days[*i as usize]
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2100() {
        assert_eq!(
            Solution::good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2),
            vec![2, 3]
        );
        assert_eq!(
            Solution::good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(
            Solution::good_days_to_rob_bank(vec![1, 2, 3, 4, 5, 6], 2),
            vec![]
        );
    }
}
