pub struct Solution {}

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut num = num;
        let mut digits = vec![];
        while 0 < num {
            digits.push(num % 10);
            num /= 10;
        }
        digits.sort_unstable_by(|a, b| b.cmp(&a));
        (0..digits.len()).fold(0, |acc, i| acc + digits[i] * 10_i32.pow((i / 2) as u32))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2578() {
        assert_eq!(Solution::split_num(1_000_000_000), 1);
        assert_eq!(Solution::split_num(4325), 59);
        assert_eq!(Solution::split_num(687), 75);
    }
}
