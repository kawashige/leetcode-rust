pub struct Solution {}

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut result = 0;
        let mut i = 0;
        let mut n = n;
        while i < 32 {
            if n & 1 << i != 0 {
                result += 1;
                if n & 1 << (i + 1) != 0 {
                    while n & 1 << (i + 1) != 0 {
                        i += 1;
                    }
                    n |= 1 << (i + 1);
                }
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2571() {
        assert_eq!(Solution::min_operations(39), 3);
        assert_eq!(Solution::min_operations(54), 3);
    }
}
