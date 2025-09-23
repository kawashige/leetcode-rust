pub struct Solution {}

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut count = 0;
        let mut prev = s.len();
        let mut result = 0;

        for i in 0..s.len() {
            if s.as_bytes()[i] == b'0' {
                continue;
            }
            if prev + 1 < i {
                result += count;
            }
            count += 1;
            prev = i;
        }

        result
            + if s.as_bytes()[s.len() - 1] == b'0' {
                count
            } else {
                0
            }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3228() {
        assert_eq!(Solution::max_operations("1001101".to_string()), 4);
        assert_eq!(Solution::max_operations("00111".to_string()), 0);
    }
}
