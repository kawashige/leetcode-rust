pub struct Solution {}

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut r_count = [0; 26];
        let mut r_chars = 0;

        for b in s.as_bytes().iter() {
            let i = (b - b'a') as usize;
            r_count[i] += 1;
            if r_count[i] == 1 {
                r_chars += 1;
            }
        }

        let mut l_count = [0; 26];
        let mut l_chars = 0;

        let mut result = 0;

        for b in s.as_bytes().iter() {
            let i = (b - b'a') as usize;
            l_count[i] += 1;
            if l_count[i] == 1 {
                l_chars += 1;
            }

            r_count[i] -= 1;
            if r_count[i] == 0 {
                r_chars -= 1;
            }

            if l_chars == r_chars {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1525() {
        assert_eq!(Solution::num_splits("aacaba".to_string()), 2);
        assert_eq!(Solution::num_splits("abcd".to_string()), 1);
    }
}
