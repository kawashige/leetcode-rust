pub struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut prev_index = [-1; 26];
        let mut max = -1;

        for (i, b) in s.as_bytes().iter().enumerate() {
            let j = (b - b'a') as usize;
            if prev_index[j] != -1 {
                max = max.max(i as i32 - prev_index[j] - 1);
            } else {
                prev_index[j] = i as i32;
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1624() {
        assert_eq!(
            Solution::max_length_between_equal_characters("mgntdygtxrvxjnwksqhxuxtrv".to_string()),
            18
        );
        assert_eq!(
            Solution::max_length_between_equal_characters("cabbac".to_string()),
            4
        );
        assert_eq!(
            Solution::max_length_between_equal_characters("aa".to_string()),
            0
        );
        assert_eq!(
            Solution::max_length_between_equal_characters("abca".to_string()),
            2
        );
        assert_eq!(
            Solution::max_length_between_equal_characters("cbzxy".to_string()),
            -1
        );
    }
}
