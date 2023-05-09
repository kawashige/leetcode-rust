pub struct Solution {}

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then(b.cmp(&a)));
        nums[k as usize - 1].clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1985() {
        assert_eq!(
            Solution::kth_largest_number(
                vec![
                    "3".to_string(),
                    "6".to_string(),
                    "7".to_string(),
                    "10".to_string()
                ],
                4
            ),
            "3".to_string()
        );
        assert_eq!(
            Solution::kth_largest_number(
                vec![
                    "2".to_string(),
                    "21".to_string(),
                    "12".to_string(),
                    "1".to_string()
                ],
                3
            ),
            "2".to_string()
        );
        assert_eq!(
            Solution::kth_largest_number(vec!["0".to_string(), "0".to_string()], 2),
            "0".to_string()
        );
    }
}
