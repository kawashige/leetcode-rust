pub struct Solution {}

impl Solution {
    pub fn byte_to_index(b: u8) -> usize {
        match b {
            b'Q' => 0,
            b'W' => 1,
            b'E' => 2,
            _ => 3,
        }
    }

    pub fn balanced_string(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut count = bytes.iter().fold([0; 4], |mut count, b| {
            count[Self::byte_to_index(*b)] += 1;
            count
        });
        let mut min = std::usize::MAX;
        let mut j = 0;

        for i in 0..bytes.len() {
            count[Self::byte_to_index(bytes[i])] -= 1;
            while j <= i && count[Self::byte_to_index(bytes[j])] < bytes.len() / 4 {
                count[Self::byte_to_index(bytes[j])] += 1;
                j += 1;
            }
            if count.iter().all(|c| c <= &(bytes.len() / 4)) {
                min = min.min(i + 1 - j);
            }
        }

        min as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1234() {
        assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
        assert_eq!(Solution::balanced_string("QQWE".to_string()), 1);
        assert_eq!(Solution::balanced_string("QQQW".to_string()), 2);
    }
}
