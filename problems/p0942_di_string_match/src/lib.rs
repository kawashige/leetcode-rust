pub struct Solution {}

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut result = Vec::with_capacity(s.len() + 1);
        result.push(0);
        let mut min: i32 = 0;
        let mut max = 0;

        for c in s.chars() {
            if c == 'I' {
                result.push(max + 1);
                max += 1;
            } else {
                result.push(min - 1);
                min -= 1;
            }
        }

        for i in 0..result.len() {
            result[i] += min.abs();
        }

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
