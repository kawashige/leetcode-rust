pub struct Solution {}

impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut reward_values = reward_values;
        reward_values.sort_unstable();
        let mut reward = vec![false; 4000];
        reward[0] = true;
        let mut result = 0;

        for r in reward_values {
            for j in 0..r {
                if reward[j as usize] {
                    reward[(j + r) as usize] = true;
                    result = result.max(j + r);
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
    fn test_3180() {
        assert_eq!(Solution::max_total_reward(vec![1, 1, 3, 3]), 4);
        assert_eq!(Solution::max_total_reward(vec![1, 6, 4, 3, 2]), 11);
    }
}
