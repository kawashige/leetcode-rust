pub struct Solution {}

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut count = [0, 0];
        count[bytes[0] as usize - 0x30] += 1;

        let mut result = 0;
        for i in 1..bytes.len() {
            if bytes[i - 1] == bytes[i] {
                count[bytes[i] as usize - 0x30] += 1;
            } else {
                result += std::cmp::min(count[0], count[1]);
                count[bytes[i] as usize - 0x30] = 1;
            }
        }
        result + std::cmp::min(count[0], count[1])
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_696() {
        assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
        assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
    }
}
