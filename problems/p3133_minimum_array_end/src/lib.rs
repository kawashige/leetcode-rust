pub struct Solution {}

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut result = x as i64;
        let n = n - 1;
        let mut i = 0;
        let mut j = 0;
        while i < 32 {
            while result & (1 << j) != 0 {
                j += 1;
            }
            if n & (1 << i) != 0 {
                result |= 1 << j;
            }
            j += 1;
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3133() {
        assert_eq!(Solution::min_end(3, 4), 6);
        assert_eq!(Solution::min_end(2, 7), 15);
    }
}
