pub struct Solution {}

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut product = 1;
        let mut sum = 0;
        let mut x = n;

        while 0 < x {
            product *= x % 10;
            sum += x % 10;
            x /= 10;
        }

        n % (product + sum) == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3622() {
        assert!(Solution::check_divisibility(99));
        assert!(!Solution::check_divisibility(23));
    }
}
