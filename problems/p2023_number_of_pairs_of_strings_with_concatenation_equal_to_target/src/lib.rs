pub struct Solution {}

impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                if target.starts_with(&nums[i]) && &target[nums[i].len()..] == &nums[j] {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2023() {
        assert_eq!(
            Solution::num_of_pairs(
                vec![
                    "777".to_string(),
                    "7".to_string(),
                    "77".to_string(),
                    "77".to_string()
                ],
                "7777".to_string()
            ),
            4
        );
        assert_eq!(
            Solution::num_of_pairs(
                vec![
                    "123".to_string(),
                    "4".to_string(),
                    "12".to_string(),
                    "34".to_string()
                ],
                "1234".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::num_of_pairs(
                vec!["1".to_string(), "1".to_string(), "1".to_string()],
                "11".to_string()
            ),
            6
        );
    }
}
