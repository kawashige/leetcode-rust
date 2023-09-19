pub struct Solution {}

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut num = num;
        while 0 < num {
            digits.push(num % 10);
            num /= 10;
        }
        digits.sort_unstable();
        (digits[0] + digits[1]) * 10 + digits[2] + digits[3]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2160() {
        assert_eq!(Solution::minimum_sum(2932), 52);
        assert_eq!(Solution::minimum_sum(4009), 13);
    }
}
