pub struct Solution {}

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() {
            if i == bits.len() - 1 && bits[i] == 0 {
                return true;
            }
            if bits[i] == 1 {
                i += 2;
            } else {
                i += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0717() {
        assert!(Solution::is_one_bit_character(vec![1, 0, 0]));
        assert!(!Solution::is_one_bit_character(vec![1, 1, 1, 0]));
    }
}
