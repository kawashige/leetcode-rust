pub struct Solution {}

impl Solution {
    pub fn count_substrings(s: String, c: char) -> i64 {
        let count = s.as_bytes().iter().filter(|i| i == &&(c as u8)).count() as i64;
        (count * (count + 1)) / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3084() {
        assert_eq!(Solution::count_substrings("abada".to_string(), 'a'), 6);
        assert_eq!(Solution::count_substrings("zzz".to_string(), 'z'), 6);
    }
}
