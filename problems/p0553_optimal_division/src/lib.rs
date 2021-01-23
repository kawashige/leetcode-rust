pub struct Solution {}

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        match nums.len() {
            1 => nums[0].to_string(),
            2 => format!("{}/{}", nums[0], nums[1]),
            _ => format!(
                "{}/({})",
                nums[0],
                nums[1..]
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("/")
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0553() {
        assert_eq!(
            Solution::optimal_division(vec![1000, 100, 10, 2]),
            "1000/(100/10/2)"
        );
    }
}
