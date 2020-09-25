pub struct Solution {}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut num_strings = nums
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        num_strings.sort_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));
        if num_strings[0] == "0" {
            return "0".to_string();
        }
        num_strings.join("")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day25() {
        assert_eq!("0".to_string(), Solution::largest_number(vec![0, 0]));
        assert_eq!("210".to_string(), Solution::largest_number(vec![10, 2]));
        assert_eq!(
            "9534330".to_string(),
            Solution::largest_number(vec![3, 30, 34, 5, 9])
        );
    }
}
