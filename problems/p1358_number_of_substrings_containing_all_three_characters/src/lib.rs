pub struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut result = 0;
        let mut count = [0; 3];
        let mut i = 0;
        let bytes = s.as_bytes();

        for b in bytes {
            count[*b as usize - 0x61] += 1;
            while i < bytes.len() && 1 < count[bytes[i] as usize - 0x61] {
                count[bytes[i] as usize - 0x61] -= 1;
                i += 1;
            }
            if count.iter().all(|c| &0 < c) {
                result += i + 1;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1358() {
        assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
        assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3);
        assert_eq!(Solution::number_of_substrings("abc".to_string()), 1);
    }
}
