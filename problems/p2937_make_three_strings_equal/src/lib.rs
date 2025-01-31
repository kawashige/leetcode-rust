pub struct Solution {}

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let mut i = 0;
        while i < s1.len()
            && i < s2.len()
            && i < s3.len()
            && s1.as_bytes()[i] == s2.as_bytes()[i]
            && s1.as_bytes()[i] == s3.as_bytes()[i]
        {
            i += 1;
        }
        if i == 0 {
            -1
        } else {
            (s1.len() + s2.len() + s3.len() - i * 3) as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2937() {
        assert_eq!(
            Solution::find_minimum_operations(
                "abc".to_string(),
                "abb".to_string(),
                "ab".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::find_minimum_operations(
                "dac".to_string(),
                "bac".to_string(),
                "cac".to_string()
            ),
            -1
        );
    }
}
