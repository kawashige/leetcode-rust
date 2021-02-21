pub struct Solution {}

impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut n = y;
        let mut count = 0;
        while x < n {
            count += 1;
            if n % 2 == 0 {
                n /= 2;
            } else {
                n += 1;
            }
        }
        count + std::cmp::min(x * 2 - n * 2, x - n)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day21() {
        assert_eq!(Solution::broken_calc(1, 1000000000), 39);
        assert_eq!(Solution::broken_calc(2, 3), 2);
        assert_eq!(Solution::broken_calc(5, 8), 2);
        assert_eq!(Solution::broken_calc(3, 10), 3);
        assert_eq!(Solution::broken_calc(1024, 1), 1023);
    }
}
