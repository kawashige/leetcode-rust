pub struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let bytes = s.as_bytes();
        let ones = bytes.iter().filter(|b| b == &&b'1').count();
        let mut zeros = 0;
        let mut max_score = 0;
        for i in 0..(bytes.len() - 1) {
            zeros += if bytes[i] == b'0' { 1 } else { 0 };
            max_score = max_score.max(zeros + ones - (i + 1 - zeros));
        }

        max_score as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1422() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
        assert_eq!(Solution::max_score("00111".to_string()), 5);
        assert_eq!(Solution::max_score("1111".to_string()), 3);
        assert_eq!(Solution::max_score("00".to_string()), 1);
    }
}
