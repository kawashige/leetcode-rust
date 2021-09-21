pub struct Solution {}

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut result = Vec::with_capacity(s.len() + 1);
        let mut lo = 0;
        let mut high = s.len() as i32;

        for c in s.chars() {
            if c == 'I' {
                result.push(lo);
                lo += 1;
            } else {
                result.push(high);
                high -= 1
            }
        }

        result.push(lo);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0942() {
        assert_eq!(
            Solution::di_string_match("IIDI".to_string()),
            vec![1, 2, 3, 0, 4]
        );
        assert_eq!(
            Solution::di_string_match("IDID".to_string()),
            vec![2, 3, 1, 4, 0]
        );
        assert_eq!(
            Solution::di_string_match("III".to_string()),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            Solution::di_string_match("DDI".to_string()),
            vec![2, 1, 0, 3]
        );
    }
}
