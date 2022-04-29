pub struct Solution {}

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut carry = 0;
        let mut count = 0;
        for (i, b) in s.as_bytes().iter().enumerate().rev() {
            match (i, b, carry) {
                (0, _, 0) => {}
                (0, _, 1) | (_, b'0', 0) => count += 1,
                (_, b'0', 1) | (_, b'1', 0) => {
                    count += 2;
                    carry = 1;
                }
                (_, b'1', 1) => {
                    count += 1;
                    carry = 1;
                }
                _ => unreachable!(),
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1404() {
        assert_eq!(Solution::num_steps("1101".to_string()), 6);
        assert_eq!(Solution::num_steps("10".to_string()), 1);
        assert_eq!(Solution::num_steps("1".to_string()), 0);
    }
}
