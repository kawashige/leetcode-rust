pub struct Solution {}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        (0..=nums.len())
            .map(|i| format!("{:0>width$b}", i, width = nums.len()))
            .find(|s| !nums.contains(s))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1980() {
        assert_eq!(
            Solution::find_different_binary_string(vec!["01".to_string(), "10".to_string()]),
            "00".to_string()
        );
        assert_eq!(
            Solution::find_different_binary_string(vec!["00".to_string(), "01".to_string()]),
            "10".to_string()
        );
        assert_eq!(
            Solution::find_different_binary_string(vec![
                "111".to_string(),
                "011".to_string(),
                "001".to_string()
            ]),
            "000".to_string()
        );
    }
}
