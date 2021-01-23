pub struct Solution {}

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut count_a = 0;
        let mut count_l = 0;
        for c in s.chars() {
            match c {
                'A' => {
                    if count_a == 1 {
                        return false;
                    }
                    count_a += 1;
                    count_l = 0;
                }
                'L' => {
                    if count_l == 2 {
                        return false;
                    }
                    count_l += 1
                }
                _ => {
                    count_l = 0;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0551() {
        assert!(Solution::check_record("LALL".to_string()));
        assert!(Solution::check_record("PPALLP".to_string()));
        assert!(!Solution::check_record("PPALLL".to_string()));
    }
}
