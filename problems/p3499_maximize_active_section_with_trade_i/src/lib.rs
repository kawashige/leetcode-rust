pub struct Solution {}

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let mut active = 0;
        let mut count = vec![0; 3];
        let mut i = 0;
        while i < s.len() && s.as_bytes()[i] == b'1' {
            i += 1;
            active += 1;
        }
        let mut j = 0;
        let mut converted = 0;
        while i < s.len() {
            if s.as_bytes()[i] == b'1' {
                active += 1;
            }
            if 0 < i && s.as_bytes()[i] != s.as_bytes()[i - 1] {
                if j == 2 {
                    converted = converted.max(count[0] + count[2]);
                    count[0] = count[2];
                    count[1] = 0;
                    count[2] = 0;
                    j = 1;
                } else if !(j == 0 && s.as_bytes()[i] == b'0') {
                    j += 1;
                }
            }
            count[j] += 1;
            i += 1;
        }
        if j == 2 {
            converted = converted.max(count[0] + count[2]);
        }

        active + converted
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3499() {
        assert_eq!(
            Solution::max_active_sections_after_trade("01".to_string()),
            1
        );
        assert_eq!(
            Solution::max_active_sections_after_trade("0100".to_string()),
            4
        );
        assert_eq!(
            Solution::max_active_sections_after_trade("1000100".to_string()),
            7
        );
        assert_eq!(
            Solution::max_active_sections_after_trade("01010".to_string()),
            4
        );
    }
}
