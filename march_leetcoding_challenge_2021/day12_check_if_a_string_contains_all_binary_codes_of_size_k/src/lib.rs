pub struct Solution {}

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let mut need = 1 << k;
        let mut got = vec![false; need as usize];
        let all_one = need - 1;
        let mut hash = 0;

        for i in 0..s.len() {
            hash = (hash << 1 & all_one) | (s.as_bytes()[i] - b'0') as i32;
            if i >= k - 1 && !got[hash as usize] {
                got[hash as usize] = true;
                need -= 1;
                if need == 0 {
                    return true;
                }
            }
        }
        false
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
