pub struct Solution {}

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut num1 = num1;
        let mut num2 = num2;
        let mut num3 = num3;

        let mut digits = Vec::new();

        while !(num1 == 0 && num2 == 0 && num3 == 0) {
            digits.push((num1 % 10).min(num2 % 10).min(num3 % 10));
            num1 /= 10;
            num2 /= 10;
            num3 /= 10;
        }

        digits.into_iter().rev().fold(0, |acc, x| acc * 10 + x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3270() {
        assert_eq!(Solution::generate_key(1, 10, 1000), 0);
        assert_eq!(Solution::generate_key(987, 879, 798), 777);
        assert_eq!(Solution::generate_key(1, 2, 3), 1);
    }
}
