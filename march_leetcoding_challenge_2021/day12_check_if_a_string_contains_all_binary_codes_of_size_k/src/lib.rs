use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        if s.len() < k {
            return false;
        }
        (0..=(s.len() - k))
            .map(|i| &s[i..(i + k)])
            .collect::<HashSet<&str>>()
            .len()
            == 2_usize.pow(k as u32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day12() {
        assert!(!Solution::has_all_codes("0110".to_string(), 4));
        assert!(Solution::has_all_codes("00110".to_string(), 2));
        assert!(Solution::has_all_codes("0110".to_string(), 1));
        assert!(!Solution::has_all_codes("0110".to_string(), 2));
        assert!(!Solution::has_all_codes("0000000001011100".to_string(), 4));
    }
}
