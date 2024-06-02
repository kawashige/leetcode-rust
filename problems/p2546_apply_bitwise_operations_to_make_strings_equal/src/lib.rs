pub struct Solution {}

impl Solution {
    pub fn count(s: &str) -> [usize; 2] {
        s.as_bytes().iter().fold([0; 2], |mut count, b| {
            count[(b - b'0') as usize] += 1;
            count
        })
    }
    pub fn make_strings_equal(s: String, target: String) -> bool {
        let s_count = Self::count(&s);
        let t_count = Self::count(&target);

        !((s_count[1] == 0 && 0 < t_count[1]) || (0 < s_count[1] && t_count[1] == 0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2546() {
        assert!(Solution::make_strings_equal(
            "1010".to_string(),
            "0110".to_string()
        ));
        assert!(!Solution::make_strings_equal(
            "11".to_string(),
            "00".to_string()
        ));
    }
}
