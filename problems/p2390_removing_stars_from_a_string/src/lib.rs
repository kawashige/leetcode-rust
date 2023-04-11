pub struct Solution {}

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        for b in s.as_bytes() {
            if b == &b'*' {
                result.pop();
            } else {
                result.push(*b as char);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2390() {
        assert_eq!(
            Solution::remove_stars("leet**cod*e".to_string()),
            "lecoe".to_string()
        );
        assert_eq!(
            Solution::remove_stars("erase*****".to_string()),
            "".to_string()
        );
    }
}
