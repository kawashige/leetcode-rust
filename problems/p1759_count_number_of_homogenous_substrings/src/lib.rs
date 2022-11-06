pub struct Solution {}

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        const M: usize = 1_000_000_007;

        let mut result = 0;
        let mut contagious = 0;

        for i in 0..s.len() {
            if 0 < i && s.as_bytes()[i - 1] != s.as_bytes()[i] {
                contagious = 0;
            }
            contagious += 1;
            result = (result + contagious) % M;
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1759() {
        assert_eq!(Solution::count_homogenous("abbcccaa".to_string()), 13);
        assert_eq!(Solution::count_homogenous("xy".to_string()), 2);
        assert_eq!(Solution::count_homogenous("zzzzz".to_string()), 15);
    }
}
