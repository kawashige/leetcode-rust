pub struct Solution {}

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut l = 0;
        let mut count = [0; 26];
        let mut result = 0;
        for i in 0..s.len() {
            count[(s.as_bytes()[i] - b'a') as usize] += 1;
            while 2 < count[(s.as_bytes()[i] - b'a') as usize] {
                count[(s.as_bytes()[l] - b'a') as usize] -= 1;
                l += 1;
            }
            result = result.max((i - l + 1) as i32);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3090() {
        assert_eq!(
            Solution::maximum_length_substring("bcbbbcba".to_string()),
            4
        );
        assert_eq!(Solution::maximum_length_substring("aaaa".to_string()), 2);
    }
}
