pub struct Solution {}

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut nums = Vec::new();
        for i in 0..number.len() {
            if number.as_bytes()[i] == digit as u8 {
                nums.push(number[..i].to_string() + &number[i + 1..]);
            }
        }
        nums.sort_unstable();
        nums.last().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2259() {
        assert_eq!(
            Solution::remove_digit("123".to_string(), '3'),
            "12".to_string()
        );
        assert_eq!(
            Solution::remove_digit("1231".to_string(), '1'),
            "231".to_string()
        );
        assert_eq!(
            Solution::remove_digit("551".to_string(), '5'),
            "51".to_string()
        );
    }
}
