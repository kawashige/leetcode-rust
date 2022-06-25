pub struct Solution {}

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut count = 0;
        let mut ones = 0;

        for b in s.as_bytes() {
            if b == &b'1' {
                ones += 1;
                count = (count + ones) % M;
            } else {
                ones = 0;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1513() {
        assert_eq!(Solution::num_sub("0110111".to_string()), 9);
        assert_eq!(Solution::num_sub("101".to_string()), 2);
        assert_eq!(Solution::num_sub("111111".to_string()), 21);
    }
}
