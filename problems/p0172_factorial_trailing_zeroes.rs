pub struct Solution {}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut result = 0;
        let mut i = 1;
        while 5 * i <= n {
            result += 1;
            let mut d = i;
            while d % 5 == 0 {
                result += 1;
                d /= 5;
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0172() {
        assert_eq!(0, Solution::trailing_zeroes(3));
        assert_eq!(1, Solution::trailing_zeroes(5));
        assert_eq!(0, Solution::trailing_zeroes(0));
        assert_eq!(6, Solution::trailing_zeroes(25));
    }
}
