pub struct Solution {}

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        if let Some(i) = (0..num.len())
            .rev()
            .find(|i| (num.as_bytes()[*i] - b'0') % 2 == 1)
        {
            num[..=i].to_string()
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1903() {
        assert_eq!(
            Solution::largest_odd_number("52".to_string()),
            "5".to_string()
        );
        assert_eq!(
            Solution::largest_odd_number("4206".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::largest_odd_number("35427".to_string()),
            "35427".to_string()
        );
    }
}
