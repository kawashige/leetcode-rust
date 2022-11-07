pub struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut change_1 = 0;
        let mut change_2 = 0;

        for i in 0..s.len() {
            if s.as_bytes()[i] - b'0' == (i % 2) as u8 {
                change_1 += 1;
            } else {
                change_2 += 1;
            }
        }

        change_1.min(change_2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1758() {
        assert_eq!(Solution::min_operations("0100".to_string()), 1);
        assert_eq!(Solution::min_operations("10".to_string()), 0);
        assert_eq!(Solution::min_operations("1111".to_string()), 2);
    }
}
