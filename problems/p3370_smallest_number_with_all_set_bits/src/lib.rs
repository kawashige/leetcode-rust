pub struct Solution {}

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut x = 1;
        while x - 1 < n {
            x *= 2;
        }
        x - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3370() {
        assert_eq!(Solution::smallest_number(5), 7);
        assert_eq!(Solution::smallest_number(10), 15);
        assert_eq!(Solution::smallest_number(3), 3);
    }
}
