pub struct Solution {}

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut result = std::i32::MIN;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                result = result.max(0);
                continue;
            }
            let mut dp = vec![false; nums[i] as usize + 1];
            dp[0] = true;
            let mut tmp = -1;
            for j in 0..queries.len() {
                if !(queries[j][0]..=queries[j][1]).contains(&(i as i32)) || nums[i] < queries[j][2]
                {
                    continue;
                }
                for k in (0..dp.len() - queries[j][2] as usize).rev() {
                    if dp[k] {
                        dp[k + queries[j][2] as usize] = true;
                    }
                }
                if dp[nums[i] as usize] {
                    tmp = j as i32 + 1;
                    break;
                }
            }
            if tmp == -1 {
                return -1;
            }
            result = result.max(tmp);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3489() {
        assert_eq!(
            Solution::min_zero_array(
                vec![2, 0, 2],
                vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
            ),
            2
        );
        assert_eq!(
            Solution::min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
            -1
        );
        assert_eq!(
            Solution::min_zero_array(
                vec![1, 2, 3, 2, 1],
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 1],
                    vec![2, 3, 2],
                    vec![3, 4, 1],
                    vec![4, 4, 1]
                ]
            ),
            4
        );
    }
}
