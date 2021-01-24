pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(" ")
            .map(|sub| sub.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0557() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
    }
}
