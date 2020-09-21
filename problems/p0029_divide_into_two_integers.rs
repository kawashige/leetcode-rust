pub struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let sign = dividend.signum() * divisor.signum();
        let mut a = (dividend as i64).abs() as i64;
        let b = (divisor as i64).abs() as i64;
        let mut result: i64 = 0;
        for i in (0..32).rev() {
            if a - (b << i) >= 0 {
                a -= b << i;
                result += 1 << i;
            }
        }
        if sign > 0 {
            std::cmp::min(std::i32::MAX as i64, result) as i32
        } else {
            (result * sign as i64) as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0029() {
        assert_eq!(1, Solution::divide(1, 1));
        assert_eq!(3, Solution::divide(10, 3));
        assert_eq!(-2, Solution::divide(7, -3));
        assert_eq!(2147483647, Solution::divide(-2147483648, -1));
        assert_eq!(-2147483648, Solution::divide(-2147483648, 1));
    }
}
